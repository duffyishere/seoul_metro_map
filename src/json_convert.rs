use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use crate::seoul_metro_map::Station;

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
struct Data {
    header: Header,
    body: Vec<BodyItem>, // 여러 개의 정보를 저장하는 벡터
}

pub fn convert(path: &String) -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    let data: Data = serde_json::from_str(&json_str)?;
    let mut stations: Vec<Station> = Vec::new();
    for body_item in &data.body {
        let station = Station {
            name: body_item.stin_nm.clone(),
            code: body_item.stin_cd.clone(),
            line_name: body_item.rout_nm.clone(),
            order: body_item.stin_cons_ordr,
        };
        stations.push(station)
    }

    Ok(stations)
}

#[cfg(test)]
mod tests {
    use crate::json_convert::convert;
    use crate::seoul_metro_map::Station;

    #[test]
    fn test_json_parsing() -> Result<(), Box<dyn std::error::Error>> {
        let res = convert(&String::from("./untitled/resources/1.json"));
        let expected = Station {
            name: String::from("연천"),
            code: String::from("100-3"),
            line_name: String::from("1호선"),
            order: 1,
        };

        assert_eq!(res.unwrap()[0], expected);

        Ok(())
    }
}