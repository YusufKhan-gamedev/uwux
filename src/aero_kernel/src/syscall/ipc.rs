/*
 * Copyright (C) 2021-2022 The Aero Project Developers.
 *
 * This file is part of The Aero Project.
 *
 * Aero is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Aero is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Aero. If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{
    userland::{
        scheduler::get_scheduler,
    },
    utils::{
        sync::{BlockQueue, Mutex},
        validate_slice_mut,
    },
};
use aero_syscall::AeroSyscallError;
use alloc::vec::Vec;
use hashbrown::HashMap;
use lazy_static::lazy_static;

struct Message {
    from: usize,
    data: Vec<u8>,
}

lazy_static! {
    static ref BLOCK_QUEUE: BlockQueue = BlockQueue::new();
    static ref MESSAGES: Mutex<HashMap<usize, Option<Message>>> = Mutex::new(HashMap::new());
}

<<<<<<< HEAD
=======
fn messagequeue_do_recieve(
    pidptr: usize,
    messageptr: usize,
    messagesiz: usize,
    msg: Message,
) -> Result<usize, AeroSyscallError> {
    let output =
        validate_slice_mut(messageptr as *mut u8, messagesiz).ok_or(AeroSyscallError::EINVAL)?;

    output[0..msg.data.len()].copy_from_slice(&msg.data);

    unsafe {
        (pidptr as *mut usize).write(msg.from);
    }
    Ok(msg.data.len())
}
>>>>>>> e1d5ca76d (aero ipc: first round of review for IPC!)
pub fn send(pid: usize, message: usize, messagesiz: usize) -> Result<usize, AeroSyscallError> {
    let payload =
        validate_slice_mut(message as *mut u8, messagesiz).ok_or(AeroSyscallError::EINVAL)?;

    let mut bqueue = BLOCK_QUEUE
        .block_on(&MESSAGES, |msg| {
            let mp = msg.get(&pid);
            match mp {
                Some(None) => true,
                _ => false,
            }
        })
        .unwrap();
    let bqueueitem = bqueue.get_mut(&pid).unwrap();
    bqueueitem.replace(Message {
        from: get_scheduler().current_task().pid().as_usize(),
        data: payload.to_vec(),
    });
    BLOCK_QUEUE.notify_complete();
    Ok(0)
}
pub fn recv(
    pidptr: usize,
    messageptr: usize,
    messagemax: usize,
    block: usize,
) -> Result<usize, AeroSyscallError> {
    let output =
        validate_slice_mut(messageptr as *mut u8, messagemax).ok_or(AeroSyscallError::EINVAL)?;
    let pid = get_scheduler().current_task().pid().as_usize();

    if block == 0 {
        // nonblocking read
        let mut msgqueue = MESSAGES.lock();
<<<<<<< HEAD
        match msgqueue.get(&pid) {
            Some(m) => match m {
                None => return Err(AeroSyscallError::EAGAIN),
                Some(_) => {
                    let m = msgqueue.remove(&pid).unwrap().unwrap();
                    output.split_at_mut(m.data.len()).0.copy_from_slice(&m.data);
                    BLOCK_QUEUE.notify_complete();
                    unsafe {
                        *(pidptr as *mut usize) = m.from;
                    }
                    return Ok(m.data.len());
                }
            },
            None => {
                // just set it up
                msgqueue.insert(pid, None);
                return Err(AeroSyscallError::EAGAIN);
=======
        match msgqueue.get_mut(&pid) {
            Some(m) => {
                let item = m
                    .queue
                    .pop_front()
                    .expect("empty message queues should always be deleted!");
                if item.data.len() > messagemax {
                    m.queue.push_front(item);
                    return Err(AeroSyscallError::E2BIG)
                }
                if m.queue.len() == 0 {
                    msgqueue
                        .remove(&pid)
                        .expect("safety violation: modification of a value behind a mutex!");
                }
                return messagequeue_do_recieve(pidptr, messageptr, messagemax, item)
            }
            None => {
                // nope
                return Err(AeroSyscallError::EAGAIN)
>>>>>>> e1d5ca76d (aero ipc: first round of review for IPC!)
            }
        }
    }

    let mut mqueue = MESSAGES.lock();
    mqueue.insert(pid, None);
    BLOCK_QUEUE.notify_complete();
    drop(mqueue);

    let mut bqueue = BLOCK_QUEUE
        .block_on(&MESSAGES, |msg| {
<<<<<<< HEAD
            let mp = msg.get(&pid);
            match mp {
                Some(Some(_)) => true,
                _ => false,
            }
        })
        .unwrap();
    let m = bqueue.remove(&pid).unwrap().unwrap();
    output.split_at_mut(m.data.len()).0.copy_from_slice(&m.data);
    BLOCK_QUEUE.notify_complete();
    unsafe {
        *(pidptr as *mut usize) = m.from;
    }
    Ok(m.data.len())
=======
            let mp = msg.get_mut(&pid);
            if let Some(mq) = mp {
                mq.queue.front().is_some()
            } else {
                false
            }
        })
        .unwrap();

    let mq = queue_map
        .get_mut(&pid)
        .expect("someone else stole our messagequeue!");
    let msg = mq
        .queue
        .pop_front()
        .expect("someone else stole our message!");
    if msg.data.len() > messagemax {
        mq.queue.push_front(msg);
        Err(AeroSyscallError::E2BIG)
    } else {
        if mq.queue.len() == 0 {
            queue_map
                .remove(&pid)
                .expect("safety violation: modification of a value behind a mutex!");
        }
        messagequeue_do_recieve(pidptr, messageptr, messagemax, msg)
    }
>>>>>>> e1d5ca76d (aero ipc: first round of review for IPC!)
}
