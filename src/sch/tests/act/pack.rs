use crate::{
    event::ActionState,
    sch::{tests::create_proc, TaskState},
    utils, Act, StmtBuild, Workflow,
};
use std::sync::Arc;
use std::sync::Mutex;

#[tokio::test]
async fn sch_act_pack_msg() {
    let ret = Arc::new(Mutex::new(false));
    let mut workflow = Workflow::new().with_step(|step| {
        step.with_id("step1").with_act(Act::pack(|act| {
            act.with_acts(|stmts| stmts.add(Act::msg(|msg| msg.with_id("msg1"))))
        }))
    });

    workflow.print();
    let (proc, scher, emitter) = create_proc(&mut workflow, &utils::longid());
    let r = ret.clone();
    emitter.on_message(move |e| {
        println!("message: {:?}", e);
        if e.is_key("msg1") {
            *r.lock().unwrap() = true;
            e.close();
        }
    });
    scher.launch(&proc);
    scher.event_loop().await;
    proc.print();
    assert_eq!(*ret.lock().unwrap(), true);
}

#[tokio::test]
async fn sch_act_pack_next() {
    let ret = Arc::new(Mutex::new(false));
    let mut workflow = Workflow::new().with_step(|step| {
        step.with_id("step1").with_act(Act::pack(|act| {
            act.with_acts(|stmts| stmts.add(Act::msg(|msg| msg.with_id("msg1"))))
                .with_next(|act| {
                    act.with_acts(|stmts| stmts.add(Act::msg(|msg| msg.with_id("msg2"))))
                })
        }))
    });

    workflow.print();
    let (proc, scher, emitter) = create_proc(&mut workflow, &utils::longid());
    let r = ret.clone();
    emitter.on_message(move |e| {
        println!("message: {:?}", e);
        if e.is_key("msg2") {
            *r.lock().unwrap() = true;
            e.close();
        }
    });
    scher.launch(&proc);
    scher.event_loop().await;
    proc.print();
    assert_eq!(*ret.lock().unwrap(), true);
}

#[tokio::test]
async fn sch_act_pack_acts() {
    let mut workflow = Workflow::new().with_step(|step| {
        step.with_id("step1").with_act(Act::pack(|act| {
            act.with_id("pack1").with_acts(|stmts| {
                stmts
                    .add(Act::msg(|msg| msg.with_id("msg1")))
                    .add(Act::req(|act| act.with_id("act1")))
            })
        }))
    });

    workflow.print();
    let (proc, scher, emitter) = create_proc(&mut workflow, &utils::longid());
    emitter.on_message(move |e| {
        println!("message: {:?}", e);
        if e.is_key("act1") && e.is_state("created") {
            e.close();
        }
    });
    scher.launch(&proc);
    scher.event_loop().await;
    proc.print();
    assert_eq!(
        proc.task_by_nid("act1").get(0).unwrap().action_state(),
        ActionState::Created
    );
    assert_eq!(
        proc.task_by_nid("pack1").get(0).unwrap().state(),
        TaskState::Running
    );
}
