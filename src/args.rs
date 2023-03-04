use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete or show videos
    Video(VideoCommand),
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The id of the entity to delete
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct VideoCommand {
    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VideoSubcommand {
    /// Create a new video
    Create(CreateVideo),

    /// Update an existing video
    Update(UpdateVideo),

    /// Delete a video
    Delete(DeleteEntity),

    /// Show all videos
    Show,
}

#[derive(Debug, Args)]
pub struct CreateVideo {
    /// The title of the video to create
    pub title: String,

    /// The description of the video to create
    pub description: String,
}

#[derive(Debug, Args)]
pub struct UpdateVideo {
    /// The id of the video to update
    pub id: i32,

    /// The title of the video
    pub title: String,

    /// The description of the video
    pub description: String,
}
