use serde_json::Value;

use crate::{
    data, sch::NodeData, utils, ModelInfo, NodeKind, PackageInfo, ProcInfo, TaskInfo, TaskState,
    Workflow,
};

#[test]
fn model_info_package() {
    let package = &data::Package {
        id: utils::shortid(),
        name: "test package".to_string(),
        size: 100,
        data: vec![0x01, 0x02],
        create_time: 0,
        update_time: 0,
        timestamp: 0,
    };
    let info: PackageInfo = package.into();
    assert_eq!(info.id, package.id);
    assert_eq!(info.name, package.name);
    assert_eq!(info.size, package.size);
    assert_eq!(info.create_time, package.create_time);
    assert_eq!(info.update_time, package.update_time);
    assert_eq!(info.timestamp, package.timestamp);
}

#[test]
fn model_info_proc() {
    let proc = &data::Proc {
        id: utils::shortid(),
        name: "test".to_string(),
        mid: "m1".to_string(),
        state: TaskState::None.into(),
        start_time: 1234,
        end_time: 2345,
        timestamp: 11111,
        model: "{}".to_string(),
        env_local: "".to_string(),
        err: None,
    };
    let info: ProcInfo = proc.into();
    assert_eq!(info.id, proc.id);
    assert_eq!(info.name, proc.name);
    assert_eq!(info.state, proc.state);
    assert_eq!(info.start_time, proc.start_time);
    assert_eq!(info.end_time, proc.end_time);
    assert_eq!(info.timestamp, proc.timestamp);
}

#[test]
fn model_info_task() {
    let workflow = Workflow::new();
    let node_data = NodeData {
        id: "nid".to_string(),
        content: crate::sch::NodeContent::Workflow(workflow),
        level: 0,
    };
    let task = data::Task {
        id: utils::shortid(),
        kind: NodeKind::Workflow.into(),
        name: "test".to_string(),
        pid: "pid".to_string(),
        tid: "tid".to_string(),
        node_data: serde_json::to_string(&node_data).unwrap(),
        state: TaskState::None.into(),
        prev: None,
        start_time: 0,
        end_time: 0,
        hooks: "{}".to_string(),
        timestamp: 0,
        data: "{}".to_string(),
        err: None,
    };
    let info: TaskInfo = task.clone().into();
    assert_eq!(info.id, task.tid);
    assert_eq!(info.r#type, task.kind);
    assert_eq!(info.name, task.name);
    assert_eq!(info.state, task.state);
    assert_eq!(info.start_time, task.start_time);
    assert_eq!(info.end_time, task.end_time);
    assert_eq!(info.timestamp, task.timestamp);
    assert_eq!(info.pid, task.pid);
    assert_eq!(info.data, task.data);
    assert_eq!(info.nid, "nid");
}

#[test]
fn model_info_model() {
    let model = data::Model {
        id: utils::longid(),
        name: "test_model".to_string(),
        ver: 1,
        size: 1245,
        create_time: 3333,
        update_time: 0,
        data: "{}".to_string(),
        timestamp: 0,
    };
    let info: ModelInfo = model.clone().into();
    assert_eq!(info.id, model.id);
    assert_eq!(info.name, model.name);
    assert_eq!(info.ver, model.ver);
    assert_eq!(info.size, model.size);
    assert_eq!(info.create_time, model.create_time);
    assert_eq!(info.update_time, model.update_time);
    assert_eq!(info.data, model.data);
}

#[test]
fn model_info_package_arr_to_value() {
    let package = &data::Package {
        id: utils::shortid(),
        name: "test package".to_string(),
        size: 100,
        data: vec![0x01, 0x02],
        create_time: 0,
        update_time: 0,
        timestamp: 0,
    };
    let info: PackageInfo = package.into();

    let mut arr: Vec<PackageInfo> = Vec::new();
    arr.push(info.clone());

    let v: Value = arr.into();
    assert!(v.is_array());

    let v = &v[0];
    assert_eq!(v.get("id").unwrap().as_str().unwrap(), info.id);
    assert_eq!(v.get("name").unwrap().as_str().unwrap(), info.name);
    assert_eq!(v.get("size").unwrap().as_u64().unwrap(), info.size as u64);
    assert_eq!(
        v.get("create_time").unwrap().as_i64().unwrap(),
        info.create_time as i64
    );
    assert_eq!(
        v.get("update_time").unwrap().as_i64().unwrap(),
        info.update_time as i64
    );
    assert_eq!(
        v.get("timestamp").unwrap().as_i64().unwrap(),
        info.timestamp as i64
    );
}

#[test]
fn model_info_proc_arr_to_value() {
    let proc = &data::Proc {
        id: utils::shortid(),
        name: "test".to_string(),
        mid: "m1".to_string(),
        state: TaskState::None.into(),
        start_time: 1234,
        end_time: 2345,
        timestamp: 11111,
        model: "{}".to_string(),
        env_local: "".to_string(),
        err: None,
    };
    let info: ProcInfo = proc.into();

    let mut arr: Vec<ProcInfo> = Vec::new();
    arr.push(info.clone());

    let v: Value = arr.into();
    assert!(v.is_array());

    let v = &v[0];
    assert_eq!(v.get("id").unwrap().as_str().unwrap(), info.id);
    assert_eq!(v.get("mid").unwrap().as_str().unwrap(), info.mid);
    assert_eq!(v.get("name").unwrap().as_str().unwrap(), info.name);
    assert_eq!(v.get("state").unwrap().as_str().unwrap(), info.state);
    assert_eq!(
        v.get("start_time").unwrap().as_i64().unwrap(),
        info.start_time as i64
    );
    assert_eq!(
        v.get("end_time").unwrap().as_i64().unwrap(),
        info.end_time as i64
    );
    assert_eq!(
        v.get("timestamp").unwrap().as_i64().unwrap(),
        info.timestamp as i64
    );
}

#[test]
fn model_info_task_arr_to_value() {
    let workflow = Workflow::new();
    let node_data = NodeData {
        id: "nid".to_string(),
        content: crate::sch::NodeContent::Workflow(workflow),
        level: 0,
    };
    let task = data::Task {
        id: utils::shortid(),
        kind: NodeKind::Workflow.into(),
        name: "test".to_string(),
        pid: "pid".to_string(),
        tid: "tid".to_string(),
        node_data: serde_json::to_string(&node_data).unwrap(),
        state: TaskState::None.into(),
        prev: None,
        start_time: 0,
        end_time: 0,
        hooks: "{}".to_string(),
        timestamp: 0,
        data: "{}".to_string(),
        err: None,
    };
    let info: TaskInfo = task.clone().into();

    let mut arr: Vec<TaskInfo> = Vec::new();
    arr.push(info.clone());

    let v: Value = arr.into();
    assert!(v.is_array());

    let v = &v[0];
    assert_eq!(v.get("id").unwrap().as_str().unwrap(), info.id);
    assert_eq!(v.get("type").unwrap().as_str().unwrap(), info.r#type);
    assert_eq!(v.get("name").unwrap().as_str().unwrap(), info.name);
    assert_eq!(v.get("pid").unwrap().as_str().unwrap(), info.pid);
    assert_eq!(v.get("state").unwrap().as_str().unwrap(), info.state);
    assert_eq!(v.get("nid").unwrap().as_str().unwrap(), info.nid);

    assert_eq!(
        v.get("start_time").unwrap().as_i64().unwrap(),
        info.start_time as i64
    );
    assert_eq!(
        v.get("end_time").unwrap().as_i64().unwrap(),
        info.end_time as i64
    );
    assert_eq!(v.get("data").unwrap().as_str().unwrap(), info.data);
}

#[test]
fn model_info_model_arr_to_value() {
    let model = data::Model {
        id: utils::longid(),
        name: "test_model".to_string(),
        ver: 1,
        size: 1245,
        create_time: 3333,
        update_time: 0,
        data: "{}".to_string(),
        timestamp: 0,
    };

    let mut arr: Vec<ModelInfo> = Vec::new();
    arr.push(model.clone().into());

    let v: Value = arr.into();

    assert!(v.is_array());

    let v = &v[0];
    assert_eq!(v.get("name").unwrap().as_str().unwrap(), model.name);
    assert_eq!(v.get("ver").unwrap().as_u64().unwrap(), model.ver as u64);
    assert_eq!(v.get("size").unwrap().as_u64().unwrap(), model.size as u64);
    assert_eq!(
        v.get("create_time").unwrap().as_i64().unwrap(),
        model.create_time as i64
    );
    assert_eq!(
        v.get("update_time").unwrap().as_i64().unwrap(),
        model.update_time as i64
    );
    assert_eq!(v.get("data").unwrap().as_str().unwrap(), model.data);
}
