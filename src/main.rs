/*
 * @Author: ZuoXichen
 * @Date: 2023-06-02 23:11:26
 * @LastEditTime: 2023-06-03 20:43:09
 * @LastEditors: ZuoXichen
 * @Description:
 */
pub mod align;

use std::{ fs::File, io::{ BufWriter, Write } };

use seq_io::fasta::{ Reader, Record };
use clap::{ self, Parser, command, Subcommand, arg };

#[derive(Parser)]
#[command(
    name = "jvman",
    author = "ZuoXichen",
    version = "0.1.0",
    about = " Mutiple Sequence Comparison",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compare {
        #[arg(short = 'n', long)]
        sequence_number: usize,
        #[arg(short, long, default_value_t = 1)]
        match_score: i32,
        #[arg(short, long, default_value_t = -2)]
        dismatch_score: i32,
        #[arg(short, long, default_value_t = -5)]
        indel_score: i32,
        #[arg(short, long)]
        fasta_path: String,
        #[arg(short, long, default_value_t = String::from(""))]
        output_path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compare {
            sequence_number,
            match_score,
            dismatch_score,
            indel_score,
            fasta_path,
            output_path,
        } => {
            let mut score_matrix = vec![match_score, dismatch_score, indel_score];
            let mut describe_vec = Vec::new();
            // println!("score matrix {:?}", score_matrix);
            let mut reader = Reader::from_path(&fasta_path).expect(
                &format!("Can not find file {}", &fasta_path)
            );
            let mut sequences_vec: Vec<String> = Vec::new();
            while let Some(record) = reader.next() {
                let record = record.expect("Error reading record");
                sequences_vec.push(String::from_utf8(record.owned_seq()).unwrap());
                describe_vec.push(String::from(record.id().unwrap()));
            }
            if sequence_number == sequences_vec.len() && sequences_vec.len() >= 2 {
                if sequence_number > 2 {
                    // println!("{:?}", sequences_vec);
                    let result = align::muscle(sequence_number, sequences_vec, score_matrix);
                    if output_path == "" {
                        println!(
                            "--------------------------------------------------------------------"
                        );
                        println!("\t原序列");
                        for it_finish in &result {
                            println!("{}", it_finish.str);
                        }
                        println!("\t比对后");
                        for it_finish in &result {
                            println!("{}", it_finish.res);
                        }
                        println!(
                            "--------------------------------------------------------------------"
                        );
                    } else {
                        let mut f = File::create(output_path).unwrap();
                        let mut buf_writer = BufWriter::new(f);
                        buf_writer
                            .write(format!("@ Sequence Alignment Result File\n").as_bytes())
                            .unwrap();
                        buf_writer
                            .write(
                                format!("@ Date: {}\n", chrono::Local::now().to_string()).as_bytes()
                            )
                            .unwrap();
                        buf_writer.write(format!("@ SAR_Version: 0.1\n").as_bytes()).unwrap();
                        buf_writer
                            .write(
                                format!("@ Aligned Sequence Number: {}\n", sequence_number).as_bytes()
                            )
                            .unwrap();
                        for i in 0..sequence_number {
                            buf_writer.write(format!(">{}\n", describe_vec[i]).as_bytes()).unwrap();
                            buf_writer.write(format!("{}\n", result[i].res).as_bytes()).unwrap();
                        }
                        buf_writer.flush().unwrap();
                    }
                } else {
                    let res = align::NeedlemanWunch(
                        &sequences_vec[0],
                        &sequences_vec[1],
                        &score_matrix
                    );
                    if output_path == "" {
                        println!(
                            "--------------------------------------------------------------------"
                        );
                        println!("原序列");

                        println!("{}\n{}", res.str1, res.str2);

                        println!("比对后");
                        println!("{}\n{}", res.res1, res.res2);
                        println!("Score: {}", res.score);
                        println!(
                            "--------------------------------------------------------------------"
                        );
                    } else {
                        let mut f = File::create(output_path).unwrap();
                        let mut buf_writer = BufWriter::new(f);
                        buf_writer
                            .write(format!("@ Sequence Alignment Result File\n").as_bytes())
                            .unwrap();
                        buf_writer
                            .write(
                                format!("@ Date: {}\n", chrono::Local::now().to_string()).as_bytes()
                            )
                            .unwrap();
                        buf_writer.write(format!("@ SAR_Version: 0.1\n").as_bytes()).unwrap();
                        buf_writer
                            .write(
                                format!("@ Aligned Sequence Number: {}\n", sequence_number).as_bytes()
                            )
                            .unwrap();
                        buf_writer
                            .write(format!("@ Alignment Score: {}\n", res.score).as_bytes())
                            .unwrap();

                        buf_writer.write(format!(">{}\n", describe_vec[0]).as_bytes()).unwrap();
                        buf_writer.write(format!("{}\n", res.res1).as_bytes()).unwrap();
                        buf_writer.write(format!(">{}\n", describe_vec[1]).as_bytes()).unwrap();
                        buf_writer.write(format!("{}\n", res.res2).as_bytes()).unwrap();
                        buf_writer.flush().unwrap();
                    }
                }
            } else {
                eprintln!("Dismatched Sequence Number or Invaild FASTA Input");
            }
        }
    };
}
