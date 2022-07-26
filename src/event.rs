use crate::User;

pub enum Event {
    /// 送礼
    Gift(Gift),
    /// 弹幕
    Comment(Comment),
    /// 点赞
    Like(Like),
    /// 分享
    Share(Share),
    /// 关注主播
    Follow(Follow),
    /// 自定义任务
    Task(Task),
}

pub struct Gift {
    /// 送礼人信息
    user: User,
    /// 礼物id
    gift_id: String,
    /// 礼物名称
    gift_name: String,
    /// 礼物总价值
    gift_total_dou: u32,
    /// 礼物个数
    count: u32,
}

pub struct Comment {
    user: User,
    /// 弹幕内容
    content: String,
}

pub struct Like {
    user: User,
    /// 点赞个数
    count: u32,
}

pub struct Share {
    user: User,
}

pub struct Follow {
    user: User,
}

pub struct Task {
    /// 完成任务的用户信息，直播间内的集体任务没有用户
    user: Option<User>,
    /// 任务id
    task_id: u32,
}
