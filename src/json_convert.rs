use std::fs::File;
use std::io::{Error, Read};
use serde::Deserialize;
use crate::metro_graph::Data;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BodyItem {
    rout_cd: String,
    rout_nm: String,
    stin_cons_ordr: i8,
    rail_opr_istt_cd: String,
    ln_cd: String,
    stin_cd: String,
    stin_nm: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Header {
    result_cnt: i32,
    result_code: String,
    result_msg: String,
}

#[derive(Debug, Deserialize)]
struct Body {
    header: Header,
    body: Vec<BodyItem>, // 여러 개의 정보를 저장하는 벡터
}

pub fn convert(path: &String) -> Result<Vec<Data>, Error> {
    let mut file = File::open(path)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    let data: Body = serde_json::from_str(&json_str)?;
    let mut stations: Vec<Data> = Vec::new();
    for body_item in &data.body {
        let station = Data {
            name: body_item.stin_nm.clone(),
            line_name: body_item.rout_nm.clone(),
        };
        stations.push(station)
    }

    Ok(stations)
}

#[cfg(test)]
mod tests {
    use crate::json_convert::convert;
    use crate::metro_graph::Data;

    #[test]
    fn test_json_parsing() -> Result<(), Box<dyn std::error::Error>> {
        let res = convert(&String::from("./untitled/resources/1.json"));
        let expected = Data::new(String::from("연천"), String::from("1호선"));
        assert_eq!(res.unwrap()[0], expected);

        Ok(())
    }
}