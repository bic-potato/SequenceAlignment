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
#[derive(Clone)]
#[derive(Debug)]
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
    goUp: i32, //是否向上回溯
    goLeftUp: i32, //是否向左上回溯
    goLeft: i32, //是否向左回溯
    score: i32, //得分矩阵第(i, j)这个单元的分值
}
impl BacktrackingUnit {
    pub fn new() -> BacktrackingUnit {
        return BacktrackingUnit { goUp: 0, goLeft: 0, goLeftUp: 0, score: 0 };
    }
}

pub fn muscle(seq_number: usize, ss: Vec<String>, score_matrix: Vec<i32>) -> Vec<SingleSeq> {
    let sequence_groups = seq_number;

    let mut queue_initial: Vec<ResUnit> = Vec::new(); //定义等待整合的队列，是一个ResUnit对象vector
    let mut queue_finish: Vec<SingleSeq> = Vec::new(); //定义整合完毕的队列，是一个String类型的vector

    let mut res: Vec<Vec<ResUnit>> = vec![Vec::new();sequence_groups];
    for i in 0..sequence_groups {
        res[i] = vec!(ResUnit::new();sequence_groups);
        for j in 0..sequence_groups {
            res[i][j] = ResUnit::new();
        }
    }

    getResUnitMatrix(ss, sequence_groups.try_into().unwrap(), &mut res, &score_matrix);

    //开始多序列比对
    //定义队列长度
    let queue_length = ((sequence_groups - 1) * sequence_groups) / 2;
    println!("queue_length:{}", queue_length);

    //将res内元素放入等待整合队列--按分数从高到低排列
    for i in 0..sequence_groups {
        for j in i + 1..sequence_groups {
            //放入容器
            let mut unit = res[i][j].clone();
            queue_initial.push(unit);
        }
    }

    queue_initial.sort_by(|a, b| b.score.cmp(&a.score));
    //排序

    //最多循环queue_length次
    for i in 0..queue_length {
        //当结果队列长度与sequence_groups相等之后，就说明全部放入了结果队列，可以跳出循环了
        // if sequence_groups == queue_finish.len()
        // {
        //     break;
        // }
        //一个ResUnit对象的str1属性和str2均不在
        if
            ifStrInQueueFinish(&queue_initial[i].str1, &queue_finish) < 0 &&
            ifStrInQueueFinish(&queue_initial[i].str2, &queue_finish) < 0
        {
            let mut singleSeq1: SingleSeq = SingleSeq::new();
            let mut singleSeq2: SingleSeq = SingleSeq::new();
            singleSeq1.str = queue_initial[i].str1.as_str().to_string();
            singleSeq1.res = queue_initial[i].res1.as_str().to_string();
            singleSeq2.str = queue_initial[i].str2.as_str().to_string();
            singleSeq2.res = queue_initial[i].res2.as_str().to_string();

            //如果结果队列已经有元素，，且又来了俩不相干的，却很匹配的序列对
            if queue_finish.len() > 0 {
                // 将结果队列第一个的序列和queue_initial[i].str1进行双序列比对
                let temp = NeedlemanWunch(
                    &queue_finish[0].str,
                    &queue_initial[i].str1,
                    &score_matrix
                );

                // println!("TEMP:{:?}",&temp);
                //进行规整操作
                queue_finish = RegularTwo(
                    &mut queue_initial[i],
                    temp,
                    &mut queue_finish
                ).to_owned();
            } else {
                queue_finish.push(singleSeq1);
                queue_finish.push(singleSeq2);
            }
        } else if
            //str1在，str2不在
            ifStrInQueueFinish(&queue_initial[i].str1, &queue_finish) > -1 &&
            ifStrInQueueFinish(&queue_initial[i].str2, &queue_finish) < 0
        {
            let item = ifStrInQueueFinish(&queue_initial[i].str1, &queue_finish);
            queue_finish = RegularSeq1(
                &mut queue_initial[i],
                &mut queue_finish,
                item.try_into().unwrap()
            ).to_owned();
        } else if
            //str2在，str1不在
            ifStrInQueueFinish(&queue_initial[i].str2, &queue_finish) > -1 &&
            ifStrInQueueFinish(&queue_initial[i].str1, &queue_finish) < 0
        {
            let item = ifStrInQueueFinish(&queue_initial[i].str2, &queue_finish);
            queue_finish = RegularSeq2(
                &mut queue_initial[i],
                &mut queue_finish,
                item.try_into().unwrap()
            ).to_owned();
        }
    }

    return queue_finish;
    //声明一个迭代器，来访问vector容器
}

/*
    规整函数，规整两个序列情况
    
    queue_finish      temp		tag
    A1				  A2		E1
    B				  E2		F
    C
    D
    */
fn RegularTwo<'a>(
    tag: &mut ResUnit,
    temp: ResUnit,
    queue_finish: &'a mut Vec<SingleSeq>
) -> &'a mut Vec<SingleSeq> {
    let mut E2 = temp.res2;
    let mut E1 = tag.res1.to_string();
    let mut A1 = queue_finish[0].res.to_string();
    let mut A2 = temp.res1;
    let mut F = tag.res2.to_string();
    let mut tempStr: String = "".to_string();

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
        for k in 0..E1.len() - j {
            tempStr += "-";
        }
        E2 += &tempStr;
        A2 += &tempStr;
    } else if j == E1.len() {
        //E1先到头
        for k in 0..E2.len() - i {
            tempStr += "-";
        }
        E1 += &tempStr;
        F += &tempStr;
    }

    //将tempStr置空
    let mut tempStr = "".to_string().to_string();

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
                for mut it in &mut *queue_finish {
                    it.res.insert(i, '-');
                }
            }
        }
    }

    if i == A1.len() {
        //A1先到头
        for k in 0..A2.len() - j {
            tempStr += "-";
        }
        A1 += &tempStr;
        for mut it in &mut *queue_finish {
            it.res += &tempStr;
        }
    } else if j == A2.len() {
        //A2先到头
        for k in 0..A1.len() - i {
            tempStr += "-";
        }
        A2 += &tempStr;
        E1 += &tempStr;
        F += &tempStr;
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

/*
    规整函数，规整序列1情况
    
    queue_finish      tag
    A1				  A2
    B				  E
    C
    D
    */
fn RegularSeq1<'a>(
    tag: &mut ResUnit,
    queue_finish: &'a mut Vec<SingleSeq>,
    item: usize
) -> &'a mut Vec<SingleSeq> {
    let main_seq = &queue_finish[item]; //找到和seq1相同的序列
    let mut A1 = main_seq.res.to_string();
    let mut A2 = tag.res1.to_string();
    let mut E = tag.res2.to_string();
    let mut tempStr = "".to_string();

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
                for mut it in &mut *queue_finish {
                    it.res.insert(i, '-');
                }
            }
        }
    }

    if i == A1.len() {
        //A1先到头
        for k in 0..A2.len() - j {
            tempStr += "-";
        }
        A1 += &tempStr;
        for mut it in &mut *queue_finish {
            it.res += &tempStr;
        }
    } else if j == A2.len() {
        //A2先到头
        for k in 0..A1.len() - i {
            tempStr += "-";
        }
        A2 += &tempStr;
        E += &tempStr;
    }

    //添加
    let mut sE: SingleSeq = SingleSeq::new();
    sE.res = E;
    sE.str = tag.str2.to_string();
    // println!("1\t{:?}",&sE);
    queue_finish.push(sE);
    return queue_finish;
}

/*
    规整函数，规整序列2情况
    
    queue_finish      tag
    A1				  E
    B				  A2
    C
    D
    */
fn RegularSeq2<'a>(
    tag: &mut ResUnit,
    queue_finish: &'a mut Vec<SingleSeq>,
    item: usize
) -> &'a mut Vec<SingleSeq> {
    let mut main_seq = &queue_finish[item]; //找到和seq1相同的序列
    let mut A1 = main_seq.res.to_string();
    let mut A2 = tag.res2.to_string();
    let mut E = tag.res1.to_string();
    let mut tempStr = "".to_string();

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
                for mut it in &mut *queue_finish {
                    it.res.insert(i, '-');
                }
            }
        }
    }

    if i == A1.len() {
        //A1先到头
        for k in 0..A2.len() - j {
            tempStr += "-";
        }
        A1 += &tempStr;
        for mut it in &mut *queue_finish {
            it.res += &tempStr;
        }
    } else if j == A2.len() {
        //A2先到头
        for k in 0..A1.len() - i {
            tempStr += "-";
        }
        A2 += &tempStr;
        E += &tempStr;
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
fn ifStrInQueueFinish(str: &str, queue_finish: &Vec<SingleSeq>) -> i32 {
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
fn getResUnitMatrix(
    s: Vec<String>,
    length: usize,
    res: &mut Vec<Vec<ResUnit>>,
    score_matrix: &Vec<i32>
) {
    let sLength = length;
    println!("sLength:{}", sLength);
    if sLength == 1 {
        println!("不符合输入规范");
    }

    for i in 0..sLength {
        for j in i + 1..sLength {
            //只遍历上三角区域
            res[i][j] = NeedlemanWunch(&s[i], &s[j], score_matrix);
        }
    }
}

/**
    比较三种路径之间谁最大
    
    f(i-1,j-1),f(i-1,j)+indel,f(i,j-1)+indel
    */
fn max3(a: i32, b: i32, c: i32) -> i32 {
    let temp = if a > b { a } else { b };
    return if temp > c { temp } else { c };
}

/**
    比较两个字符类型属于什么，match，dismatch，indel
    */
fn CompareChar(a: char, b: char, score_matrix: &Vec<i32>) -> i32 {
    let MATCH: i32 = score_matrix[0];
    let DIS_MATCH: i32 = score_matrix[1];
    let INDEL: i32 = score_matrix[2];
    if a == b {
        return MATCH;
    } else if a == ' ' || b == ' ' {
        return INDEL;
    } else {
        return DIS_MATCH;
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
    mut resUnit: &'a mut ResUnit
) -> &'a mut ResUnit {
    const INDEL_CHAR: char = '-';
    let temp = &item[i][j];
    // println!("traceback 1:{}, 2:{}, tag:{}",i,j,n);
    if resUnit.tag != 1 {
        // println!("1");
        // println!("i:{}, j:{}, tag:{}",i,j,n);
        // println!("{}",i!=0 && j!=0 );
        if i == 0 && j == 0 {
            // 到矩阵单元(0, 0)才算结束，这代表初始的两个字符串的每个字符都被比对到了
            // println!("2");
            resUnit.str1 = str1.to_string();
            resUnit.str2 = str2.to_string();
            resUnit.res1 = res1.to_string();
            resUnit.res2 = res2.to_string();
            resUnit.tag = 1;
            return resUnit;
        }
        if temp.goUp != 0 {
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
            resUnit = traceback(
                item,
                i - 1,
                j,
                str1,
                str2,
                res1.to_string(),
                res2.to_string(),
                n + 1,
                resUnit
            );
        }
        if temp.goLeftUp != 0 {
            // 向左上回溯一格
            // println!("4");
            let mut res1 =
                str1
                    .chars()
                    .nth(i - 1)
                    .unwrap()
                    .to_string() + &res1;
            let mut res2 =
                str2
                    .chars()
                    .nth(j - 1)
                    .unwrap()
                    .to_string() + &res2;
            // println!("{} {}", &str1.chars().nth(i - 1 ).unwrap().to_string(), &res2);
            resUnit = traceback(
                item,
                i - 1,
                j - 1,
                str1,
                str2,
                res1.to_string(),
                res2.to_string(),
                n + 1,
                resUnit
            );
        }
        if temp.goLeft != 0 {
            // 向左回溯一格
            // println!("5");
            res1 = INDEL_CHAR.to_string() + &res1;
            res2 =
                str2
                    .chars()
                    .nth(j - 1)
                    .unwrap()
                    .to_string() + &res2;
            resUnit = traceback(item, i, j - 1, str1, str2, res1, res2, n + 1, resUnit);
        }
        // println!("6");
        return resUnit;
    } else {
        return resUnit;
    }
}

pub fn NeedlemanWunch(str1: &str, str2: &str, score_matrix: &Vec<i32>) -> ResUnit {
    let INDEL: i32 = score_matrix[2];
    //字符串str1,str2长度
    let m = str1.len();
    let n = str2.len();

    let (mut m1, mut m2, mut m3, mut mm) = (0, 0, 0, 0);

    let mut unit: Vec<Vec<BacktrackingUnit>> = vec!(vec!(BacktrackingUnit::new();n+1);m+1);

    // 初始化

    for i in 0..m {
        for j in 0..n {
            unit[i][j].goUp = 0;
            unit[i][j].goLeftUp = 0;
            unit[i][j].goLeft = 0;
        }
    }
    unit[0][0].score = 0;
    for i in 1..m + 1 {
        unit[i][0].score = INDEL * (i as i32);
        unit[i][0].goUp = 1;
    }
    for j in 1..n + 1 {
        unit[0][j].score = INDEL * (j as i32);
        unit[0][j].goLeft = 1;
    }

    // 动态规划算法计算得分矩阵每个单元的分值
    for i in 1..m + 1 {
        for j in 1..n + 1 {
            m1 = unit[i - 1][j].score + INDEL;
            m2 =
                unit[i - 1][j - 1].score +
                CompareChar(
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
            m3 = unit[i][j - 1].score + INDEL;
            mm = max3(m1, m2, m3);
            unit[i][j].score = mm;
            //判断路径来源
            if m1 == mm {
                unit[i][j].goUp = 1;
            }
            if m2 == mm {
                unit[i][j].goLeftUp = 1;
            }
            if m3 == mm {
                unit[i][j].goLeft = 1;
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

pub fn two_seq_compare(ss: Vec<String>, score_matrix: Vec<i32>) {
    let res = NeedlemanWunch(&ss[0], &ss[1], &score_matrix);

}
