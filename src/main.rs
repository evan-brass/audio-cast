use serde::Deserialize;
use csv;
use chrono::prelude::*;
use std::{
	error::Error,
	io::{
		prelude::*
	}, 
	fs::OpenOptions
};

#[derive(Debug, Deserialize)]
struct Episode {
	id: u32,
	title: String,
	description: String,
	url: String,
	mime: String,
	published: String,
	size: usize,
	duration: String,
	author: String
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut output = OpenOptions::new().write(true).truncate(true).create(true).open("feed.xml")?;
	let mut episodes = csv::ReaderBuilder::new().has_headers(true).from_path("episodes.csv")?;

	// Write the prelude which includes information about the podcast.
	let published = Local.from_local_datetime(&NaiveDate::from_ymd(2018, 06, 17).and_hms(0, 0, 0)).single().expect("").to_rfc2822();
	write!(output, "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<rss xmlns:itunes=\"http://www.itunes.com/dtds/podcast-1.0.dtd\" xmlns:atom=\"http://www.w3.org/2005/Atom\" version=\"2.0\">
	<channel>
		<link>{origin}</link>
		<language>en-us</language>
		<copyright>{author} copyright {cp_year}</copyright>
		<image>
			<url>{origin}logo.jpg</url>
			<title>{title}</title>
			<link>{origin}</link>
		</image>
		<pubDate>{published}</pubDate>
		<title>{title}</title>
		<description>{description}</description>
		<lastBuildDate>{gen}</lastBuildDate>
        <atom:link href=\"{origin}feed.xml\" rel=\"self\" type=\"application/rss+xml\"/>
        <itunes:author>{author}</itunes:author>
        <itunes:summary>
            {description}
        </itunes:summary>
        <itunes:subtitle>{subtitle}</itunes:subtitle>
        <itunes:owner>
            <itunes:name>{author}</itunes:name>
            <itunes:email>{email}</itunes:email>
        </itunes:owner>
        <itunes:explicit>No</itunes:explicit>
        <itunes:keywords>
            awkward, books,
        </itunes:keywords>
        <itunes:category text=\"{category}\"/>
		", 
		origin="https://evan-brass.github.io/audio-cast/", 
		title="Silhouette", 
		description="Reading books, maybe the news, maybe other stuff.", 
		subtitle="Books and stuff.",
		author="(Mostly) Evan Brass",
		email="brassevan@gmail.com",
		category="Technology",
		published=published, gen=Local::now().to_rfc2822(), cp_year=Local::now().format("%Y")
	)?;

	for result in episodes.deserialize() {
		let episode: Episode = result?;

		let published = {
			let month = episode.published[0..2].parse::<u32>()?;
			let day = episode.published[3..5].parse::<u32>()?;
			let year = episode.published[6..10].parse::<i32>()?;

			let date = Local.from_local_datetime(
				&NaiveDate::from_ymd(year, month, day).and_hms(0, 0, 0)
			).single().expect("");

			date.to_rfc2822()
		};

		write!(output, "
		<item>
			<link>{}</link>
			<title>{}</title>
			<description>{}</description>
			<itunes:summary>{}</itunes:summary>
			<enclosure url=\"{}\" type=\"{}\" length=\"{}\" />
			<guid>{}</guid>
			<itunes:duration>{}</itunes:duration>
			<pubDate>{}</pubDate>
			<itunes:keywords>
				
			</itunes:keywords>
			<itunes:explicit>no</itunes:explicit>
		</item>
		", episode.url, episode.title, episode.description, episode.description, episode.url, episode.mime, episode.size, episode.url, episode.duration, published)?;
	}

	// Write the postlude which closes any tags from the prelude
	write!(output, "
	</channel>
</rss>
	")?;

	Ok(())
}
