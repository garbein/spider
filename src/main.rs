extern crate env_logger;
extern crate log;
extern crate reqwest;
extern crate regex;
extern crate mysql;

use reqwest::Response;
use regex::Regex;
use std::{thread, time};

struct QyCompany {
    _company_id: i32,
    qy_id: i32,
    qy_name: Option<String>,
}
fn main() {
    env_logger::init();
    println!("starting");
    let mut i = 0;
    loop {
        if i >= 100 {
            break;
        }
        run();
        let ten_millis = time::Duration::from_secs(2);
        thread::sleep(ten_millis);
        i = i + 1;
    }
    println!("ending");
}

fn run() {
    let pool = mysql::Pool::new("mysql://root:2iUy,pUir5E9@localhost:3306/spider").unwrap();
    let rows: Vec<QyCompany> =
    pool.prep_exec("SELECT company_id, qy_id, qy_name from qy_company where company_id = 0 and data_empty = 0  order by id limit 10", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {
            let (company_id, qy_id, qy_name) = mysql::from_row(row);
            QyCompany {
                _company_id: company_id,
                qy_id: qy_id,
                qy_name: qy_name,
            }
        }).collect()
    }).unwrap();
    for x in &rows {
        let qy_name = match &x.qy_name {
            Some(v) => v.as_str(),
            None => "",
        };
        println!("{} {}", x.qy_id, qy_name);
        let company_id = search(qy_name);
        let update_sql;
        if company_id != 0 {
            update_sql = format!("update qy_company set company_id = '{}' where qy_id = '{}'", company_id, x.qy_id);
        } else {
            update_sql = format!("update qy_company set data_empty = '1' where qy_id = '{}'", x.qy_id);
        }
        pool.prep_exec(update_sql, ()).unwrap();
    }
}

fn search(name: &str) -> i32 {
    let url = format!("https://m.51job.com/search/joblist.php?keyword={}&keywordtype=2&jobarea=000000", name);
    println!("{}",url);
    let mut response = http_get(url.as_str());
    let content = response.text().unwrap();
    let s = content.as_str();
    let re = Regex::new(r#"(<b class="jobid" value="(?P<job_id>[0-9]+)" jobtype="0"></b><h3><span>(?P<job_name>.*?)</span></h3>)"#).unwrap();
    
    let mut company_id = 0;
    for cap in re.captures_iter(s) {
        //println!("{:?}", cap);
        //println!("{}", &cap["job_id"]);
        //println!("{}", &cap["job_name"]);
        company_id = item(&cap["job_id"]);
    }
    if company_id == 0 {
        return company_id;
    }
    println!("{}", company_id);
    category(company_id);
    company_id
    /*
    let caps = re.captures(s).unwrap();
    println!("{:?}", &caps["job_id"]);
    println!("{:?}", &caps["job_name"]);
    */
}

fn category(company_id: i32) {
    let url = format!("https://m.51job.com/jobs/shanghai/co{}.html", company_id);
    println!("{}",url);
    let mut response = http_get(url.as_str());
    let content = response.text().unwrap();
    let s = content.as_str();
    let re_str = r#"<h3>(?P<company_name>.*?)</h3>[\s\S]*?<span class="s_w">(?P<company_type>.*?)</span>(<span class="s_r">(?P<people>.*?)</span>)?<span class="s_g">(?P<industry>.*?)</span>[\s\S]*?(上班地址 : (?P<address>.*?)</span></a>)?\s+</span></a>[\s\S]*?<article>(?P<info>[\s\S]*?)</article>"#;
    let re = Regex::new(re_str).unwrap();
    let caps = re.captures(s).unwrap();
    let raw_info = match caps.name("info") {
        Some(v) => v.as_str(),
        None => "",
    };
    let people = match caps.name("people") {
        Some(v) => v.as_str(),
        None => "",
    };
    let address = match caps.name("address") {
        Some(v) => v.as_str(),
        None => "",
    };
    let info = raw_info.replace("'", r"\'");
    let sql = format!("insert into company (company_id,company_name,people,company_type,industry,address,info) values \
    ('{}','{}','{}','{}','{}','{}','{}')",
    company_id, &caps["company_name"], people, &caps["company_type"], &caps["industry"], address, info);
    let pool = mysql::Pool::new("mysql://root:2iUy,pUir5E9@localhost:3306/spider").unwrap();
    pool.prep_exec(sql, ()).unwrap();
}

fn item(job_id: &str) -> i32 {
    let url = format!("https://m.51job.com/jobs/shanghai/{}.html", job_id);
    println!("{}", url);
    let mut response = http_get(url.as_str());
    let content = response.text().unwrap();
    let s = content.as_str();
    //println!("{}", s);
    let re_str = r#"<div class="jt">\s*<p>(?P<job_name>.*)</p>\s*<span>(?P<publish_date>.*?)</span>\s*<em>(?P<region>.*?)</em>[\s\S]*?<p class="jp">(?P<salary>.*?)</p>[\s\S]*?<span class="s_r">(?P<number>.*?)</span>(<span class="s_n">(?P<years>.*?)</span>)?(<span class="s_x">(?P<education>.*?)(</span><span class="s_y">(?P<english>.*))?</span>)?(<span> |		</div>)[\s\S]*?/co(?P<company_id>\d+)\.html[\s\S]*?(class="arr a2"><span>上班地址 : (?P<address>.*?)</span></a>)?\s+</div>[\s\S]*?<article>(?P<info>[\s\S]*?)</article>"#;
    let re = Regex::new(re_str).unwrap();
    let caps = re.captures(s).unwrap();
    let salary = match caps.name("salary") {
        Some(v) => v.as_str(),
        None => "",
    };
    let number = match caps.name("number") {
        Some(v) => v.as_str(),
        None => "",
    };
    let years = match caps.name("years") {
        Some(v) => v.as_str(),
        None => "",
    };
    let education = match caps.name("education") {
        Some(v) => v.as_str(),
        None => "",
    };
    let english = match caps.name("english") {
        Some(v) => v.as_str(),
        None => "",
    };
    let address = match caps.name("address") {
        Some(v) => v.as_str(),
        None => "",
    };
    let raw_info = match caps.name("info") {
        Some(v) => v.as_str(),
        None => "",
    };
    let info = raw_info.replace("'", r"\'");
    //info = info.replace(r#"""#, r#"\""#);
    let sql = format!("insert into job (job_id,company_id,job_name,publish_date,region,salary,number,years,education,english,address,info) values \
    ('{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}')\
    ", job_id, &caps["company_id"], &caps["job_name"], &caps["publish_date"], &caps["region"], salary, number, years, education, english, address, info);
    let pool = mysql::Pool::new("mysql://root:2iUy,pUir5E9@localhost:3306/spider").unwrap();
    pool.prep_exec(sql, ()).unwrap();
    let company_id: i32 = caps["company_id"].parse().unwrap();
    company_id
}

fn http_get(url: &str) -> Response {
    reqwest::get(url).unwrap()
}
