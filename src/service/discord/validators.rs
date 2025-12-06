use std::sync::LazyLock;

use super::types::serenity_component::{ComponentType, Component, ActionRowComponent};


/// Validates a set of components
pub fn validate_components(rows: &[Component]) -> Result<(), crate::Error> {
    const MAX_BUTTONS_PER_ACTION_ROW: usize = 5;
    const MAX_SELECTS_PER_ACTION_ROW: usize = 1;
    const MAX_POSSIBLE_COMPONENTS: usize = 5; // 5 action rows, each with 5 components

    if rows.len() > MAX_POSSIBLE_COMPONENTS {
        return Err(format!("Too many components, limit is {MAX_POSSIBLE_COMPONENTS}").into());
    }

    for row in rows.iter() {
        let row = match row {
            Component::ActionRow(ar) => ar,
            _ => continue // Ignore non-action row components
        };

        if row.kind != ComponentType::ActionRow {
            return Err("Invalid component type, must be an action row".into());
        }

        // Validate the action row
        let mut num_buttons = 0;
        let mut num_selects = 0;

        for component in row.components.iter() {
            match component {
                ActionRowComponent::Button(_) => {
                    if num_buttons >= MAX_BUTTONS_PER_ACTION_ROW {
                        return Err(format!(
                            "Too many buttons in action row, limit is {MAX_BUTTONS_PER_ACTION_ROW}",
                        )
                        .into());
                    }
                    if num_selects > 0 {
                        return Err("Cannot have buttons and a select menu in action row".into());
                    }
                    num_buttons += 1;
                }
                ActionRowComponent::SelectMenu(_) => {
                    if num_selects >= MAX_SELECTS_PER_ACTION_ROW {
                        return Err(format!(
                            "Too many select menus in action row, limit is {MAX_SELECTS_PER_ACTION_ROW}",
                        )
                        .into());
                    }

                    if num_buttons > 0 {
                        return Err("Cannot have buttons and a select menu in action row".into());
                    }

                    num_selects += 1;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

/// Validates an embed, returning total number of characters used
///
/// NOTE: This function does not check for offensive words (as embeds are currently deemed safe)
pub fn validate_embed(embed: &super::types::CreateEmbed) -> Result<usize, crate::Error> {
    const EMBED_TITLE_LIMIT: usize = 256;
    const EMBED_DESCRIPTION_LIMIT: usize = 4096;
    const EMBED_FOOTER_TEXT_LIMIT: usize = 2048;
    const EMBED_AUTHOR_NAME_LIMIT: usize = 256;
    const EMBED_FIELD_NAME_LIMIT: usize = 256;
    const EMBED_FIELD_VALUE_LIMIT: usize = 1024;

    let mut total_chars = 0;

    // Validate title
    if let Some(title) = &embed.title {
        if title.is_empty() {
            return Err("Embed title cannot be empty".into());
        }

        //validate_string(title)?;

        if title.len() > EMBED_TITLE_LIMIT {
            return Err(format!("Embed title is too long, limit is {EMBED_TITLE_LIMIT}").into());
        }

        total_chars += title.len();
    }

    // Validate description
    if let Some(description) = &embed.description {
        if description.is_empty() {
            return Err("Embed description cannot be empty".into());
        }

        //validate_string(description)?;

        if description.len() > EMBED_DESCRIPTION_LIMIT {
            return Err(format!(
                "Embed description is too long, limit is {EMBED_DESCRIPTION_LIMIT}",
            )
            .into());
        }

        total_chars += description.len();
    }

    // Validate footer
    if let Some(footer) = &embed.footer {
        if footer.text.is_empty() {
            return Err("Embed footer text cannot be empty".into());
        }

        //validate_string(&footer.text)?;

        if footer.text.len() > EMBED_FOOTER_TEXT_LIMIT {
            return Err(format!(
                "Embed footer text is too long, limit is {EMBED_FOOTER_TEXT_LIMIT}",
            )
            .into());
        }

        total_chars += footer.text.len();
    }

    // Validate author
    if let Some(author) = &embed.author {
        if author.name.is_empty() {
            return Err("Embed author name cannot be empty".into());
        }

        //validate_string(&author.name)?;

        if author.name.len() > EMBED_AUTHOR_NAME_LIMIT {
            return Err(format!(
                "Embed author name is too long, limit is {EMBED_AUTHOR_NAME_LIMIT}",
            )
            .into());
        }

        total_chars += author.name.len();
    }

    // Validate fields
    for field in embed.fields.iter() {
        if field.name.is_empty() {
            return Err("Embed field name cannot be empty".into());
        }

        //validate_string(&field.name)?;

        if field.name.len() > EMBED_FIELD_NAME_LIMIT {
            return Err(
                format!("Embed field name is too long, limit is {EMBED_FIELD_NAME_LIMIT}",).into(),
            );
        }

        total_chars += field.name.len();

        if field.value.is_empty() {
            return Err("Embed field value cannot be empty".into());
        }

        //validate_string(&field.value)?;

        if field.value.len() > EMBED_FIELD_VALUE_LIMIT {
            return Err(format!(
                "Embed field value is too long, limit is {EMBED_FIELD_VALUE_LIMIT}",
            )
            .into());
        }

        total_chars += field.value.len();
    }

    Ok(total_chars)
}

/// Validates all messages
pub fn validate_message(message: &super::types::CreateMessage) -> Result<(), crate::Error> {
    pub const MESSAGE_CONTENT_LIMIT: usize = 2000;
    pub const MAX_EMBED_CHARACTERS_LIMIT: usize = 6000;

    let has_content = message.content.is_some();
    let has_embed = !message.embeds.is_empty();
    let has_attachments = message.attachments.is_some()
        && !message
            .attachments
            .as_ref()
            .unwrap()
            .new_and_existing_attachments
            .is_empty();
    let has_poll = message.poll.is_some();
    let has_sticker_ids = !message.sticker_ids.is_empty();
    let has_components =
        message.components.is_some() && !message.components.as_ref().unwrap().is_empty();

    if !has_content
        && !has_embed
        && !has_attachments
        && !has_poll
        && !has_sticker_ids
        && !has_components
    {
        return Err("No content/embeds/attachments/poll/sticker_ids/components set".into());
    }

    if let Some(content) = message.content.as_ref() {
        if content.is_empty() {
            return Err("Message content cannot be empty".into());
        }

        //validate_string(content)?;

        if content.len() > MESSAGE_CONTENT_LIMIT {
            return Err(
                format!("Message content is too long, limit is {MESSAGE_CONTENT_LIMIT}",).into(),
            );
        }
    }

    // Validate all embeds
    let mut total_embed_chars = 0;

    for embed in message.embeds.iter() {
        total_embed_chars += validate_embed(embed)?;

        if total_embed_chars > MAX_EMBED_CHARACTERS_LIMIT {
            return Err(format!(
                "Total embed characters is too long, limit is {MAX_EMBED_CHARACTERS_LIMIT}",
            )
            .into());
        }
    }

    // Validate components
    if let Some(components) = message.components.as_ref() {
        validate_components(components)?
    }

    Ok(())
}

/// Validates all messages
pub fn validate_message_edit(message: &super::types::EditMessage) -> Result<(), crate::Error> {
    pub const MESSAGE_CONTENT_LIMIT: usize = 2000;
    pub const MAX_EMBED_CHARACTERS_LIMIT: usize = 6000;

    let has_content = message.content.is_some();
    let has_embed = if let Some(embeds) = message.embeds.as_ref() {
        !embeds.is_empty()
    } else {
        false
    };
    let has_attachments = message.attachments.is_some()
        && !message
            .attachments
            .as_ref()
            .unwrap()
            .new_and_existing_attachments
            .is_empty();
    let has_components =
        message.components.is_some() && !message.components.as_ref().unwrap().is_empty();

    if !has_content
        && !has_embed
        && !has_attachments
        && !has_components
    {
        return Err("No content/embeds/attachments/components set".into());
    }

    if let Some(content) = message.content.as_ref() {
        if content.is_empty() {
            return Err("Message content cannot be empty".into());
        }

        //validate_string(content)?;

        if content.len() > MESSAGE_CONTENT_LIMIT {
            return Err(
                format!("Message content is too long, limit is {MESSAGE_CONTENT_LIMIT}",).into(),
            );
        }
    }

    // Validate all embeds
    let mut total_embed_chars = 0;

    if let Some(embeds) = message.embeds.as_ref() {
        for embed in embeds.iter() {
            total_embed_chars += validate_embed(embed)?;

            if total_embed_chars > MAX_EMBED_CHARACTERS_LIMIT {
                return Err(format!(
                    "Total embed characters is too long, limit is {MAX_EMBED_CHARACTERS_LIMIT}",
                )
                .into());
            }
        }
    }

    // Validate components
    if let Some(components) = message.components.as_ref() {
        validate_components(components)?
    }

    Ok(())
}

/// Validates webhook execute
pub fn validate_webhook_execute(message: &super::types::ExecuteWebhook) -> Result<(), crate::Error> {
    pub const MESSAGE_CONTENT_LIMIT: usize = 2000;
    pub const MAX_EMBED_CHARACTERS_LIMIT: usize = 6000;
    pub const MAX_WEBHOOK_USERNAME_LIMIT: usize = 80; 

    let has_content = message.content.is_some();
    let has_embed = if let Some(embeds) = message.embeds.as_ref() {
        !embeds.is_empty()
    } else {
        false
    };
    let has_attachments = message.attachments.is_some()
        && !message
            .attachments
            .as_ref()
            .unwrap()
            .new_and_existing_attachments
            .is_empty();
    let has_components =
        message.components.is_some() && !message.components.as_ref().unwrap().is_empty();
    let has_poll = message.poll.is_some();

    if !has_content
        && !has_embed
        && !has_attachments
        && !has_poll
        && !has_components
    {
        return Err("No content/embeds/attachments/poll/components set".into());
    }

    if let Some(content) = message.content.as_ref() {
        if content.is_empty() {
            return Err("Message content cannot be empty".into());
        }

        //validate_string(content)?;

        if content.len() > MESSAGE_CONTENT_LIMIT {
            return Err(
                format!("Message content is too long, limit is {MESSAGE_CONTENT_LIMIT}",).into(),
            );
        }
    }

    // Validate all embeds
    let mut total_embed_chars = 0;

    if let Some(embeds) = message.embeds.as_ref() {
        for embed in embeds.iter() {
            total_embed_chars += validate_embed(embed)?;

            if total_embed_chars > MAX_EMBED_CHARACTERS_LIMIT {
                return Err(format!(
                    "Total embed characters is too long, limit is {MAX_EMBED_CHARACTERS_LIMIT}",
                )
                .into());
            }
        }
    }

    // Validate components
    if let Some(components) = message.components.as_ref() {
        validate_components(components)?
    }

    // Validate username
    if let Some(username) = message.username.as_ref() {
        if username.is_empty() {
            return Err("Webhook username cannot be empty".into());
        }

        //validate_string(username)?;

        if username.len() > MAX_WEBHOOK_USERNAME_LIMIT {
            return Err(
                format!("Webhook username is too long, limit is {MAX_WEBHOOK_USERNAME_LIMIT}",).into(),
            );
        }
    }

    Ok(())
}

fn validate_option(
    option: &super::types::CreateCommandOption,
    kind: serenity::all::CommandType,
    depth: u8,
) -> Result<(), crate::Error> {
    if depth > 3 {
        // Prevent nested options (which are not supported by Discord itself)
        return Err("Exceeded maximum depth of 3 for command options".into());
    }

    // For CHAT_INPUT commands, validate against Discord's regex
    if kind == serenity::all::CommandType::ChatInput {
        // Validate the name against Discord's regex for CHAT_INPUT commands
        validate_name_option_chatinput(&option.name)?;
    }

    // Check for name localizations
    if let Some(name_localizations) = option.name_localizations.as_ref() {
        for (_lang, name) in name_localizations.iter() {
            // For CHAT_INPUT commands, validate against Discord's regex
            if kind == serenity::all::CommandType::ChatInput {
                validate_name_option_chatinput(name)?;
            }
        }
    }

    for option in option.options.iter() {
        validate_option(option, kind, depth + 1)?;
    }

    Ok(())
}

pub fn validate_command(command: &super::types::CreateCommand) -> Result<(), crate::Error> {
    let kind = command
        .kind
        .unwrap_or(serenity::all::CommandType::ChatInput);

    if let Some(name) = command.fields.name.as_ref() {
        // For CHAT_INPUT commands, validate against Discord's regex
        if kind == serenity::all::CommandType::ChatInput {
            validate_name_option_chatinput(name)?;
        }
    }

    for (_lang, name) in command.fields.name_localizations.iter() {
        // For CHAT_INPUT commands, validate against Discord's regex
        if kind == serenity::all::CommandType::ChatInput {
            validate_name_option_chatinput(name)?;
        }
    }

    for option in command.fields.options.iter() {
        validate_option(option, kind, 1)?;
    }

    Ok(())
}

static DISCORD_NAME_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    // Compile the regex for Discord name validation
    regex::Regex::new(r"^[-_'\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$")
        .expect("Failed to compile regex for Discord name validation")
});

/// Discord provides the regex `^[-_'\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$` for validating names in CHAT_INPUT commands
pub fn validate_name_option_chatinput(name: &str) -> Result<(), crate::Error> {
    // Check if the name matches the Discord regex for CHAT_INPUT commands
    if DISCORD_NAME_REGEX.is_match(name) {
        Ok(())
    } else {
        // Return an error if it doesn't match
        Err(format!("Name '{name}' does not match Discord's regex for CHAT_INPUT commands",).into())
    }
}

#[cfg(test)]
mod test_chatinput {
    use super::*;

    /// Basic test to validate the `validate_name_option_chatinput` function
    #[test]
    fn test_validate_name_option_chatinput() {
        assert!(validate_name_option_chatinput("valid_name").is_ok());
        assert!(validate_name_option_chatinput("valid-name").is_ok());
        assert!(validate_name_option_chatinput("valid_name123").is_ok());
        assert!(validate_name_option_chatinput("valid'name").is_ok());
        assert!(validate_name_option_chatinput("valid'name_123").is_ok());
        assert!(validate_name_option_chatinput("valid-name_123").is_ok());

        assert!(validate_name_option_chatinput("invalid name").is_err());
        assert!(validate_name_option_chatinput("invalid@name").is_err());
        assert!(validate_name_option_chatinput(
            "too_long_name_that_exceeds_the_limit_of_thirty_two_characters"
        )
        .is_err());
    }
}
