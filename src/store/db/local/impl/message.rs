use crate::{
    store::{
        db::local::{DbColumn, DbRow, DbSchema, DbType},
        Message,
    },
    Result,
};
use rusqlite::{types::Value, Error as DbError, Result as DbResult, Row};
impl DbSchema for Message {
    fn schema() -> Result<Vec<(String, DbColumn)>> {
        let mut map = Vec::new();
        map.push((
            "id".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                is_primary_key: true,
                ..Default::default()
            },
        ));
        map.push((
            "name".to_string(),
            DbColumn {
                db_type: DbType::Text,
                ..Default::default()
            },
        ));
        map.push((
            "tid".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                is_index: true,
                ..Default::default()
            },
        ));
        map.push((
            "state".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                is_primary_key: false,
                ..Default::default()
            },
        ));
        map.push((
            "type".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                ..Default::default()
            },
        ));
        map.push((
            "source".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                ..Default::default()
            },
        ));
        map.push((
            "model".to_string(),
            DbColumn {
                db_type: DbType::Text,
                ..Default::default()
            },
        ));

        map.push((
            "pid".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                is_index: true,
                ..Default::default()
            },
        ));
        map.push((
            "nid".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                is_index: true,
                ..Default::default()
            },
        ));
        map.push((
            "mid".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                is_index: true,
                ..Default::default()
            },
        ));
        map.push((
            "key".to_string(),
            DbColumn {
                db_type: DbType::Text,
                ..Default::default()
            },
        ));
        map.push((
            "inputs".to_string(),
            DbColumn {
                db_type: DbType::Text,
                ..Default::default()
            },
        ));
        map.push((
            "outputs".to_string(),
            DbColumn {
                db_type: DbType::Text,
                ..Default::default()
            },
        ));
        map.push((
            "tag".to_string(),
            DbColumn {
                db_type: DbType::Text,
                ..Default::default()
            },
        ));
        map.push((
            "start_time".to_string(),
            DbColumn {
                db_type: DbType::Int64,
                ..Default::default()
            },
        ));
        map.push((
            "end_time".to_string(),
            DbColumn {
                db_type: DbType::Int64,
                ..Default::default()
            },
        ));
        map.push((
            "chan_id".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                ..Default::default()
            },
        ));
        map.push((
            "chan_pattern".to_string(),
            DbColumn {
                db_type: DbType::Text,
                is_not_null: true,
                ..Default::default()
            },
        ));
        map.push((
            "create_time".to_string(),
            DbColumn {
                db_type: DbType::Int64,
                ..Default::default()
            },
        ));
        map.push((
            "update_time".to_string(),
            DbColumn {
                db_type: DbType::Int64,
                ..Default::default()
            },
        ));
        map.push((
            "status".to_string(),
            DbColumn {
                db_type: DbType::Int8,
                is_index: true,
                ..Default::default()
            },
        ));
        map.push((
            "retry_times".to_string(),
            DbColumn {
                db_type: DbType::Int32,
                ..Default::default()
            },
        ));
        map.push((
            "timestamp".to_string(),
            DbColumn {
                db_type: DbType::Int64,
                ..Default::default()
            },
        ));

        Ok(map)
    }
}

impl DbRow for Message {
    fn id(&self) -> &str {
        &self.id
    }

    fn from_row<'a>(row: &Row<'a>) -> DbResult<Message, DbError> {
        Ok(Message {
            id: row.get::<usize, String>(0).unwrap(),
            name: row.get::<usize, String>(1).unwrap(),
            tid: row.get::<usize, String>(2).unwrap(),
            state: row.get::<usize, String>(3).unwrap(),
            r#type: row.get::<usize, String>(4).unwrap(),
            source: row.get::<usize, String>(5).unwrap(),
            model: row.get::<usize, String>(6).unwrap(),
            pid: row.get::<usize, String>(7).unwrap(),
            nid: row.get::<usize, String>(8).unwrap(),
            mid: row.get::<usize, String>(9).unwrap(),
            key: row.get::<usize, String>(10).unwrap(),
            inputs: row.get::<usize, String>(11).unwrap(),
            outputs: row.get::<usize, String>(12).unwrap(),
            tag: row.get::<usize, String>(13).unwrap(),
            start_time: row.get::<usize, i64>(14).unwrap(),
            end_time: row.get::<usize, i64>(14).unwrap(),
            chan_id: row.get::<usize, String>(16).unwrap(),
            chan_pattern: row.get::<usize, String>(17).unwrap(),
            create_time: row.get::<usize, i64>(18).unwrap().into(),
            update_time: row.get::<usize, i64>(19).unwrap().into(),
            status: row.get::<usize, i8>(20).unwrap().into(),
            retry_times: row.get::<usize, i32>(21).unwrap().into(),
            timestamp: row.get::<usize, i64>(22).unwrap().into(),
        })
    }

    fn to_values(&self) -> Result<Vec<(String, Value)>> {
        let mut ret = Vec::new();

        ret.push(("id".to_string(), Value::Text(self.id.clone())));
        ret.push(("name".to_string(), Value::Text(self.name.clone())));
        ret.push(("tid".to_string(), Value::Text(self.tid.clone())));
        ret.push(("state".to_string(), Value::Text(self.state.clone())));
        ret.push(("type".to_string(), Value::Text(self.r#type.clone())));
        ret.push(("source".to_string(), Value::Text(self.source.clone())));
        ret.push(("model".to_string(), Value::Text(self.model.clone())));
        ret.push(("pid".to_string(), Value::Text(self.pid.clone())));
        ret.push(("nid".to_string(), Value::Text(self.nid.clone())));
        ret.push(("mid".to_string(), Value::Text(self.mid.clone())));
        ret.push(("key".to_string(), Value::Text(self.key.clone())));
        ret.push(("inputs".to_string(), Value::Text(self.inputs.clone())));
        ret.push(("outputs".to_string(), Value::Text(self.outputs.clone())));
        ret.push(("tag".to_string(), Value::Text(self.tag.clone())));
        ret.push(("start_time".to_string(), Value::Integer(self.start_time)));
        ret.push(("end_time".to_string(), Value::Integer(self.end_time)));
        ret.push(("chan_id".to_string(), Value::Text(self.chan_id.clone())));
        ret.push((
            "chan_pattern".to_string(),
            Value::Text(self.chan_pattern.clone()),
        ));
        ret.push(("create_time".to_string(), Value::Integer(self.create_time)));
        ret.push(("update_time".to_string(), Value::Integer(self.update_time)));
        ret.push(("status".to_string(), Value::Integer(self.status.into())));
        ret.push((
            "retry_times".to_string(),
            Value::Integer(self.retry_times as i64),
        ));
        ret.push(("timestamp".to_string(), Value::Integer(self.timestamp)));

        Ok(ret)
    }
}
