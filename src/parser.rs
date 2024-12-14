use regex::Regex;
use std::rc::Rc;
use std::sync::LazyLock;

pub static INFO_MESSAGE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(.*)\] \[Render thread/INFO\]: (.*)").unwrap());

pub const SYSTEM_CHAT_TOKEN: &str = "[System] [CHAT] ";
pub const CHAT_TOKEN: &str = "[CHAT] ";
pub const EMOTE_TOKEN: &str = "* ";
pub const NICKNAME_OPEN_BRACKET: char = '<';
pub const NICKNAME_CLOSE_BRACKET: char = '>';

pub struct Settings {
    pub included_names: Rc<str>,
    pub excluded_names: Rc<str>,
}

#[derive(Debug)]
pub struct InfoLine {
    pub time: String,
    pub content: String,
}

#[derive(Debug)]
pub struct ChatMessage {
    pub time: String,
    pub message: String,
    pub source: ChatMessageSource,
}

#[derive(Debug)]
pub enum ChatMessageSource {
    Named(String),
    Emote,
    System,
}

pub fn parse_log(log: String) {
    let info_lines = extract_info_lines(log);
}

pub fn extract_info_lines(log: String) -> Vec<InfoLine> {
    let parsed_lines = INFO_MESSAGE_REGEX.captures_iter(&log).map(|x| x.extract());
    parsed_lines
        .map(|x| {
            let (_, [time, content]) = x;
            InfoLine {
                time: time.to_string(),
                content: content.to_string(),
            }
        })
        .collect()
}

pub fn extract_chat_messages(log: String) -> Vec<ChatMessage> {
    let info_lines = extract_info_lines(log);

    info_lines
        .into_iter()
        .filter_map(info_line_to_chat_message)
        .collect()
}

pub fn info_line_to_chat_message(line: InfoLine) -> Option<ChatMessage> {
    let time = line.time;

    if let Some(message) = line.content.strip_prefix(SYSTEM_CHAT_TOKEN) {
        return Some(ChatMessage {
            time,
            message: message.to_owned(),
            source: ChatMessageSource::System,
        });
    };

    let not_system_content = line.content.strip_prefix(CHAT_TOKEN)?;

    if let Some(message) = not_system_content.strip_prefix(EMOTE_TOKEN) {
        return Some(ChatMessage {
            time,
            message: message.to_owned(),
            source: ChatMessageSource::Emote,
        });
    };

    if let Some((name, message)) = not_system_content
        .strip_prefix(NICKNAME_OPEN_BRACKET)?
        .split_once(NICKNAME_CLOSE_BRACKET)
    {
        return Some(ChatMessage {
            time,
            message: message.trim_start().to_owned(),
            source: ChatMessageSource::Named(name.to_string()),
        });
    };

    None
}

#[cfg(test)]
mod tests {
    use crate::parser::extract_chat_messages;

    use super::extract_info_lines;

    #[test]
    fn test_extract_info_lines() {
        let output = extract_info_lines(TEST_LOG.to_owned());
        println!("{:#?}", output);
    }

    #[test]
    fn test_extract_chat_messages() {
        let output = extract_chat_messages(TEST_LOG.to_owned());
        println!("{:#?}", output);
    }

    // FIXME i want to use a better example for this that i can show people
    const TEST_LOG: &str = "\
[19:08:12] [Render thread/INFO]: [CHAT] * Abigail pockets the lantern as she waits for Azure to return, as well as returning the pen to the space it was retrieved from and smudging away the rectangle.
[19:08:21] [Render thread/INFO]: [CHAT] <EDDIE> holy fuck youre a pixel
[19:08:24] [Render thread/INFO]: [CHAT] <Azure Caraway> 'm gonna go give this to Abigail :3!
[19:08:30] [Render thread/INFO]: [System] [CHAT] [Debug]: Hitboxes: shown
[19:09:02] [Render thread/INFO]: [System] [CHAT] Set own game mode to Spectator Mode
[19:09:05] [Render thread/INFO]: [System] [CHAT] Set own game mode to Creative Mode
[19:09:09] [Render thread/INFO]: [CHAT] <Azure Caraway> gots it!
[19:09:09] [Render thread/INFO]: [System] [CHAT] [Debug]: Hitboxes: hidden
[19:09:47] [Render thread/INFO]: [CHAT] <EDDIE> har har harhar har
[19:10:19] [Render thread/INFO]: [CHAT] <EDDIE> holy fuck do your eyes sway or am i crazy
[19:10:28] [Render thread/INFO]: [CHAT] <EDDIE> thats so swag
[19:10:29] [Render thread/INFO]: [CHAT] <kathrynne> when standing
[19:10:33] [Render thread/INFO]: [System] [CHAT] Set own game mode to Spectator Mode
[19:10:34] [Render thread/INFO]: [System] [CHAT] Set own game mode to Creative Mode
[19:10:44] [Render thread/INFO]: [CHAT] <Abigail> Lovely! Thank you Azure
[19:10:56] [Render thread/INFO]: [CHAT] <kathrynne> theyre just like a pair of glasses
[19:10:57] [Render thread/INFO]: [CHAT] <EDDIE> i changed my zoom button and my primary action button and its fucking me up
[19:11:06] [Render thread/INFO]: [CHAT] <Azure Caraway> yeah of course.! [she hands this over as well]
[19:11:10] [Render thread/INFO]: [CHAT] <EDDIE> oh my yay
[19:11:20] [Render thread/INFO]: [CHAT] <EDDIE> dude we're so fucking short
[19:11:39] [Render thread/INFO]: [CHAT] <EDDIE> rot sized workshop
[19:11:49] [Render thread/INFO]: [CHAT] <Abigail> With that, I believe I will be on my way!
[19:12:12] [Render thread/INFO]: [CHAT] <kathrynne> im planning on redesigning her to have hologram eyes instead because its just easier but im too attached to her glasses/eye screens having secondary motion for the cpm model LOL
[19:12:19] [Render thread/INFO]: [CHAT] <kathrynne> it looks very goofy and fun
[19:12:25] [Render thread/INFO]: [CHAT] <EDDIE> OUUU....its so fun
[19:12:35] [Render thread/INFO]: [CHAT] <EDDIE> ineed to live there
[19:12:53] [Render thread/INFO]: [CHAT] <EDDIE> my zoom button
[19:12:56] [Render thread/INFO]: [System] [CHAT] Entity kathrynne was granted the power Phantom Form from source apoli:command.
[19:12:56] [Render thread/INFO]: [System] [CHAT] Entity kathrynne was granted the power Phasing from source apoli:command.
[19:12:56] [Render thread/INFO]: [System] [CHAT] Entity kathrynne had the power Leap revoked.
[19:13:02] [Render thread/INFO]: [CHAT] <EDDIE> IOASHDIUASHDJASHOJD
[19:13:11] [Render thread/WARN]: Received passengers for unknown entity
[19:13:26] [Render thread/INFO]: [System] [CHAT] Set own game mode to Spectator Mode
[19:13:28] [Render thread/WARN]: Received passengers for unknown entity
[19:13:29] [Render thread/WARN]: Pixels array length (0) is smaller than canvas area (512)
[19:13:31] [Render thread/WARN]: Received passengers for unknown entity
[19:13:36] [Render thread/WARN]: Received passengers for unknown entity
[19:13:38] [Render thread/WARN]: Received passengers for unknown entity
[19:13:43] [Render thread/INFO]: [CHAT] <Abigail> Take care now
[19:13:45] [Render thread/WARN]: Received passengers for unknown entity
[19:13:55] [Render thread/INFO]: [CHAT] <Azure Caraway> i will! um, good luck i souppose
[19:13:57] [Render thread/WARN]: Received passengers for unknown entity
[19:13:59] [Render thread/INFO]: [CHAT] <EDDIE> circle of live
[19:14:00] [Render thread/INFO]: [CHAT] <EDDIE> fi
[19:14:01] [Render thread/WARN]: Received passengers for unknown entity
[19:14:01] [Render thread/INFO]: [CHAT] <EDDIE> life
[19:14:03] [Render thread/WARN]: Class xaero.common.minimap.render.radar.EntityIconModelPartsRenderer$VertexConsumerWrapper does not support optimized vertex writing code paths, which may cause reduced rendering performance
[19:14:14] [Render thread/WARN]: Received passengers for unknown entity
[19:14:18] [Render thread/WARN]: Received passengers for unknown entity
[19:14:27] [Render thread/WARN]: Received passengers for unknown entity
[19:14:32] [Render thread/INFO]: [CHAT] <EDDIE> please call it roommate pride flag
[19:14:36] [Render thread/WARN]: Received passengers for unknown entity
[19:14:42] [Render thread/INFO]: [CHAT] <EDDIE> bro we're so tall
[19:14:44] [Render thread/WARN]: Received passengers for unknown entity
[19:14:49] [Render thread/WARN]: Received passengers for unknown entity
[19:14:52] [Render thread/INFO]: [System] [CHAT] Dimitri left the game
    ";
}
