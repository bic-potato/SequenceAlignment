#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct ResUnit { //一次双序列比对后的结果
    pub str1: String, //原始序列1
    pub str2: String, //原始序列2
    pub res1: String, //结果序列1
    pub res2: String, //结果序列2
    pub score: i32, //序列总得分，反映两个序列的相似程度
    tag: i32, //禁止迭代多次
}

impl ResUnit {
    pub fn new() -> ResUnit {
        return ResUnit {
            str1: "".to_string(),
            str2: "".to_string(),
            res1: "".to_string(),
            res2: "".to_string(),
            score: 0,
            tag: 0,
        };
    }
}
#[derive(Clone, Debug)]
pub struct SingleSeq { //一个序列被整合后的样子
    pub str: String, //一个序列的原始序列
    pub res: String, //一个序列被整合后的样子
}

impl SingleSeq {
    pub fn new() -> SingleSeq {
        return SingleSeq { str: "".to_string(), res: "".to_string() };
    }
}

#[derive(Clone)]
pub struct BacktrackingUnit {
    go_up: bool, //是否向上回溯
    go_left_up: bool, //是否向左上回溯
    go_left: bool, //是否向左回溯
    score: i32, //得分矩阵第(i, j)这个单元的分值
}
impl BacktrackingUnit {
    pub fn new() -> BacktrackingUnit {
        return BacktrackingUnit { go_up: false, go_left: false, go_left_up: false, score: 0 };
    }
}

/**
多序列比对主函数入口
*/
pub fn muscle(seq_number: usize, ss: Vec<String>, score_matrix: Vec<i32>) -> Vec<SingleSeq> {
    let sequence_groups = seq_number;

    let mut queue_initial: Vec<ResUnit> = Vec::new(); //定义等待整合的队列，是一个ResUnit对象vector
    let mut queue_finish: Vec<SingleSeq> = Vec::new(); //定义整合完毕的队列，是一个SingleSeq类型的vector

    let mut res: Vec<Vec<ResUnit>> = vec![Vec::new();sequence_groups];
    for i in 0..sequence_groups {
        res[i] = vec!(ResUnit::new();sequence_groups);
        for j in 0..sequence_groups {
            res[i][j] = ResUnit::new();
        }
    }

    get_res_unit_matrix(ss, sequence_groups.try_into().unwrap(), &mut res, &score_matrix);

    //开始多序列比对
    //定义队列长度
    let queue_length = ((sequence_groups - 1) * sequence_groups) / 2;
    println!("queue_length:{}", queue_length);

    //将res内元素放入等待整合队列--按分数从高到低排列
    for i in 0..sequence_groups {
        for j in i + 1..sequence_groups {
            //放入容器
            let unit = res[i][j].clone();
            queue_initial.push(unit);
        }
    }

    queue_initial.sort_by(|a, b| b.score.cmp(&a.score)); //排序

    for i in 0..queue_length {
        if
            if_str_in_queue_finish(&queue_initial[i].str1, &queue_finish) < 0 &&
            if_str_in_queue_finish(&queue_initial[i].str2, &queue_finish) < 0
        {
            let mut single_seq1: SingleSeq = SingleSeq::new();
            let mut single_seq2: SingleSeq = SingleSeq::new();
            single_seq1.str = queue_initial[i].str1.as_str().to_string();
            single_seq1.res = queue_initial[i].res1.as_str().to_string();
            single_seq2.str = queue_initial[i].str2.as_str().to_string();
            single_seq2.res = queue_initial[i].res2.as_str().to_string();

            //如果结果队列已经有元素，，且又来了俩不相干的，却很匹配的序列对
            if queue_finish.len() > 0 {
                // 将结果队列第一个的序列和queue_initial[i].str1进行双序列比对
                let temp = needleman_wunch(
                    &queue_finish[0].str,
                    &queue_initial[i].str1,
                    &score_matrix
                );

                // println!("TEMP:{:?}",&temp);
                //进行规整操作
                queue_finish = regular_two(
                    &mut queue_initial[i],
                    temp,
                    &mut queue_finish
                ).to_owned();
            } else {
                queue_finish.push(single_seq1);
                queue_finish.push(single_seq2);
            }
        } else if
            //str1在，str2不在
            if_str_in_queue_finish(&queue_initial[i].str1, &queue_finish) > -1 &&
            if_str_in_queue_finish(&queue_initial[i].str2, &queue_finish) < 0
        {
            let item = if_str_in_queue_finish(&queue_initial[i].str1, &queue_finish);
            queue_finish = regular_seq1(
                &mut queue_initial[i],
                &mut queue_finish,
                item.try_into().unwrap()
            ).to_owned();
        } else if
            //str2在，str1不在
            if_str_in_queue_finish(&queue_initial[i].str2, &queue_finish) > -1 &&
            if_str_in_queue_finish(&queue_initial[i].str1, &queue_finish) < 0
        {
            let item = if_str_in_queue_finish(&queue_initial[i].str2, &queue_finish);
            queue_finish = regular_seq2(
                &mut queue_initial[i],
                &mut queue_finish,
                item.try_into().unwrap()
            ).to_owned();
        }
    }

    return queue_finish;
}

/**
    规整函数，规整两个序列情况
    */
fn regular_two<'a>(
    tag: &mut ResUnit,
    temp: ResUnit,
    queue_finish: &'a mut Vec<SingleSeq>
) -> &'a mut Vec<SingleSeq> {

    let mut E2 = temp.res2;
    let mut E1 = tag.res1.to_string();
    let mut A1 = queue_finish[0].res.to_string();
    let mut A2 = temp.res1;
    let mut F = tag.res2.to_string();
    let mut temp_str = "".to_string();

    let mut i = 0;
    let mut j = 0;
    //第一步，，整合tag与temp
    while E2 != E1 && j < E1.len() && i < E2.len() {
        if E2.chars().nth(i).unwrap() == E1.chars().nth(j).unwrap() {
            i += 1;
            j += 1;
        } else {
            if E2.chars().nth(i).unwrap() == '-' {
                E1.insert(j, '-');
                F.insert(j, '-');
            } else if E1.chars().nth(j).unwrap() == '-' {
                E2.insert(i, '-');
                A2.insert(i, '-');
            }
        }
    }

    if i == E2.len() {
        //E2先到头
        for _ in 0..E1.len() - j {
            temp_str += "-";
        }
        E2 += &temp_str;
        A2 += &temp_str;
    } else if j == E1.len() {
        //E1先到头
        for _ in 0..E2.len() - i {
            temp_str += "-";
        }
        E1 += &temp_str;
        F += &temp_str;
    }

    //将tempStr置空
    let mut temp_str = "".to_string().to_string();

    //第二步 融合进queue_finish
    let mut i = 0;
    let mut j = 0;
    while A1 != A2 && i < A1.len() && j < A2.len() {
        if A1.chars().nth(i).unwrap() == A2.chars().nth(j).unwrap() {
            i += 1;
            j += 1;
        } else {
            if A1.chars().nth(i).unwrap() == '-' {
                A2.insert(j, '-');
                E1.insert(j, '-');
                F.insert(j, '-');
            } else if A2.chars().nth(j).unwrap() == '-' {
                A1.insert(i, '-');
                for it in &mut *queue_finish {
                    it.res.insert(i, '-');
                }
            }
        }
    }

    if i == A1.len() {
        //A1先到头
        for _ in 0..A2.len() - j {
            temp_str += "-";
        }
        A1 += &temp_str;
        for it in &mut *queue_finish {
            it.res += &temp_str;
        }
    } else if j == A2.len() {
        //A2先到头
        for _ in 0..A1.len() - i {
            temp_str += "-";
        }
        A2 += &temp_str;
        E1 += &temp_str;
        F += &temp_str;
    }

    //规划好之后，，将 E F 插入queue_finish尾部
    let mut sE: SingleSeq = SingleSeq::new();
    let mut sF: SingleSeq = SingleSeq::new();
    // println!("E1: {}",&E1);
    sE.res = E1;
    sE.str = tag.str1.to_string();
    sF.res = F;
    sF.str = tag.str2.to_string();
    // println!("Two\t{:?}",&sE);
    // println!("Two\t{:?}",&sE);
    queue_finish.push(sE);
    queue_finish.push(sF);
    return queue_finish;
}

/**
    规整函数，规整序列1情况
    
    queue_finish      tag
    A1				  A2
    B				  E
    C
    D
    */
fn regular_seq1<'a>(
    tag: &mut ResUnit,
    queue_finish: &'a mut Vec<SingleSeq>,
    item: usize
) -> &'a mut Vec<SingleSeq> {
    let main_seq = &queue_finish[item]; //找到和seq1相同的序列
    let mut A1 = main_seq.res.to_string();
    let mut A2 = tag.res1.to_string();
    let mut E = tag.res2.to_string();
    let mut temp_str = "".to_string();

    let mut i = 0;
    let mut j = 0;
    while A1 != A2 && i < A1.len() && j < A2.len() {
        if A1.chars().nth(i).unwrap() == A2.chars().nth(j).unwrap() {
            i += 1;
            j += 1;
        } else {
            if A1.chars().nth(i).unwrap() == '-' {
                A2.insert(j, '-');
                E.insert(j, '-');
            } else if A2.chars().nth(j).unwrap() == '-' {
                //遍历queue_finish,给queue_finish内res洗头
                A1.insert(i, '-');
                for it in &mut *queue_finish {
                    it.res.insert(i, '-');
                }
            }
        }
    }

    if i == A1.len() {
        //A1先到头
        for _ in 0..A2.len() - j {
            temp_str += "-";
        }
        A1 += &temp_str;
        for it in &mut *queue_finish {
            it.res += &temp_str;
        }
    } else if j == A2.len() {
        //A2先到头
        for _ in 0..A1.len() - i {
            temp_str += "-";
        }
        A2 += &temp_str;
        E += &temp_str;
    }

    //添加
    let mut sE: SingleSeq = SingleSeq::new();
    sE.res = E;
    sE.str = tag.str2.to_string();
    // println!("1\t{:?}",&sE);
    queue_finish.push(sE);
    return queue_finish;
}

/**
    规整函数，规整序列2情况
    
    queue_finish      tag
    A1				  E
    B				  A2
    C
    D
    */
fn regular_seq2<'a>(
    tag: &mut ResUnit,
    queue_finish: &'a mut Vec<SingleSeq>,
    item: usize
) -> &'a mut Vec<SingleSeq> {
    let main_seq = &queue_finish[item]; //找到和seq1相同的序列
    let mut A1 = main_seq.res.to_string();
    let mut A2 = tag.res2.to_string();
    let mut E = tag.res1.to_string();
    let mut temp_str = "".to_string();

    let mut i = 0;
    let mut j = 0;
    while A1 != A2 && i < A1.len() && j < A2.len() {
        if A1.chars().nth(i).unwrap() == A2.chars().nth(j).unwrap() {
            i += 1;
            j += 1;
        } else {
            if A1.chars().nth(i).unwrap() == '-' {
                A2.insert(j, '-');
                E.insert(j, '-');
            } else if A2.chars().nth(j).unwrap() == '-' {
                //遍历queue_finish,给queue_finish内res洗头
                A1.insert(i, '-');
                for it in &mut *queue_finish {
                    it.res.insert(i, '-');
                }
            }
        }
    }

    if i == A1.len() {
        //A1先到头
        for _ in 0..A2.len() - j {
            temp_str += "-";
        }
        A1 += &temp_str;
        for it in &mut *queue_finish {
            it.res += &temp_str;
        }
    } else if j == A2.len() {
        //A2先到头
        for _ in 0..A1.len() - i {
            temp_str += "-";
        }
        A2 += &temp_str;
        E += &temp_str;
    }
    //添加
    let mut sE: SingleSeq = SingleSeq::new();
    sE.res = E;
    sE.str = tag.str2.to_string();
    // println!("{:?}",&sE);
    queue_finish.push(sE);
    return queue_finish;
}

//判断一个str是否有与queue_finish数组对象内的seq相等的,没有返回-1,有就返回序号
fn if_str_in_queue_finish(str: &str, queue_finish: &Vec<SingleSeq>) -> i32 {
    let mut i = 0;
    for it in queue_finish {
        if str == it.str {
            return i;
        }
        i += 1;
    }
    return -1;
}

/**
    循环比较一组序列的值，返回一个ResUnit对象数组，二维，且是个倒三角形状
    其中，s是一个字符串类型的数组，存储等待序列比对的一组数据
    */
fn get_res_unit_matrix(
    s: Vec<String>,
    length: usize,
    res: &mut Vec<Vec<ResUnit>>,
    score_matrix: &Vec<i32>
) {
    let s_length = length;
    println!("s_Length:{}", s_length);
    if s_length == 1 {
        println!("不符合输入规范");
    }

    for i in 0..s_length {
        for j in i + 1..s_length {
            //只遍历上三角区域
            res[i][j] = needleman_wunch(&s[i], &s[j], score_matrix);
        }
    }
}

/**
    比较三种路径之间谁最大
    
    f(i-1,j-1),f(i-1,j)+indel,f(i,j-1)+indel
    */
fn max_of_3(a: i32, b: i32, c: i32) -> i32 {
    let temp = if a > b { a } else { b };
    return if temp > c { temp } else { c };
}

/**
    比较两个字符类型属于什么，match，dismatch，indel
    */
fn compare_char(a: char, b: char, score_matrix: &Vec<i32>) -> i32 {
    let matched: i32 = score_matrix[0];
    let dis_match: i32 = score_matrix[1];
    let indel: i32 = score_matrix[2];
    if a == b {
        return matched;
    } else if a == ' ' || b == ' ' {
        return indel;
    } else {
        return dis_match;
    }
}

fn traceback<'a>(
    item: &Vec<Vec<BacktrackingUnit>>,
    i: usize,
    j: usize,
    str1: &str,
    str2: &str,
    mut res1: String,
    mut res2: String,
    n: i32,
    mut res_unit: &'a mut ResUnit
) -> &'a mut ResUnit {
    const INDEL_CHAR: char = '-';
    let temp = &item[i][j];
    // println!("Traceback 1:{}, 2:{}, tag:{}",i,j,n);
    if res_unit.tag != 1 {
        // println!("1");
        // println!("i:{}, j:{}, tag:{}",i,j,n);
        // println!("{}",i!=0 && j!=0 );
        if i == 0 && j == 0 {
            // 到矩阵单元(0, 0)才算结束，这代表初始的两个字符串的每个字符都被比对到了
            // println!("2");
            res_unit.str1 = str1.to_string();
            res_unit.str2 = str2.to_string();
            res_unit.res1 = res1.to_string();
            res_unit.res2 = res2.to_string();
            res_unit.tag = 1;
            return res_unit;
        }
        if temp.go_up {
            // 向上回溯一格
            // println!("3");
            res1 =
                str1
                    .chars()
                    .nth(i - 1)
                    .unwrap()
                    .to_string() + &res1;
            res2 = INDEL_CHAR.to_string() + &res2;
            // println!("{} {}", &str1.chars().nth(i - 1 ).unwrap().to_string(), INDEL_CHAR.to_string() + &res2);
            res_unit = traceback(
                item,
                i - 1,
                j,
                str1,
                str2,
                res1.to_string(),
                res2.to_string(),
                n + 1,
                res_unit
            );
        }
        if temp.go_left_up {
            // 向左上回溯一格
            // println!("4");
            let res1 =
                str1
                    .chars()
                    .nth(i - 1)
                    .unwrap()
                    .to_string() + &res1;
            let res2 =
                str2
                    .chars()
                    .nth(j - 1)
                    .unwrap()
                    .to_string() + &res2;
            // println!("{} {}", &str1.chars().nth(i - 1 ).unwrap().to_string(), &res2);
            res_unit = traceback(
                item,
                i - 1,
                j - 1,
                str1,
                str2,
                res1.to_string(),
                res2.to_string(),
                n + 1,
                res_unit
            );
        }
        if temp.go_left {
            // 向左回溯一格
            // println!("5");
            res1 = INDEL_CHAR.to_string() + &res1;
            res2 =
                str2
                    .chars()
                    .nth(j - 1)
                    .unwrap()
                    .to_string() + &res2;
            res_unit = traceback(item, i, j - 1, str1, str2, res1, res2, n + 1, res_unit);
        }
        // println!("6");
        return res_unit;
    } else {
        return res_unit;
    }
}

pub fn needleman_wunch(str1: &str, str2: &str, score_matrix: &Vec<i32>) -> ResUnit {
    let indel: i32 = score_matrix[2];
    //字符串str1,str2长度
    let m = str1.len();
    let n = str2.len();
    let mut unit: Vec<Vec<BacktrackingUnit>> = vec!(vec!(BacktrackingUnit::new();n+1);m+1);

    // 初始化

    for i in 0..m {
        for j in 0..n {
            unit[i][j].go_up = false;
            unit[i][j].go_left_up = false;
            unit[i][j].go_left = false;
        }
    }
    unit[0][0].score = 0;
    for i in 1..m + 1 {
        unit[i][0].score = indel * (i as i32);
        unit[i][0].go_up = true;
    }
    for j in 1..n + 1 {
        unit[0][j].score = indel * (j as i32);
        unit[0][j].go_left = true;
    }

    // 动态规划算法计算得分矩阵每个单元的分值
    for i in 1..m + 1 {
        for j in 1..n + 1 {
            let score_up = unit[i - 1][j].score + indel;
            let score_left_up =
                unit[i - 1][j - 1].score +
                compare_char(
                    str1
                        .chars()
                        .nth(i - 1)
                        .unwrap(),
                    str2
                        .chars()
                        .nth(j - 1)
                        .unwrap(),
                    score_matrix
                );
            let score_left = unit[i][j - 1].score + indel;
            let score_max = max_of_3(score_left, score_left_up, score_up);
            unit[i][j].score = score_max;
            //判断路径来源
            if score_up == score_max {
                unit[i][j].go_up = true;
            }
            if score_left_up == score_max {
                unit[i][j].go_left_up = true;
            }
            if score_left == score_max {
                unit[i][j].go_left = true;
            }
            // println!("score {},{},{},{}", m1, m2, m3, mm)
        }
    }

    //开始回溯
    let mut res: ResUnit = ResUnit::new();
    res.tag = 0;
    traceback(&unit, m, n, str1, str2, "".to_string(), "".to_string(), 0, &mut res);
    res.score = unit[m][n].score;

    //返回值
    return res;
}
