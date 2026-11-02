# mp3-organizer

Set of tools to manage MP3 files:

- mp3-organizer - moves/clusters files based on ID3 tags falling back to filename structure. (Not good enough, use the "cluster" tool in https://picard.musicbrainz.org/ instead)
- mp3-find-untagged - finds all mp3 files in the given dir, searching for those with Unknown Artist, Unknown Album and Unknown Title, useful to locate input files for https://picard.musicbrainz.org/
- mp3-dedup - WIP/deprecated, use https://crates.io/crates/dupsonic instead
- mp3-stamp - WIP: takes either artist name, album or track and apply any of those as ID3 tags in the target MP3 file
- mp3 - one tool to rule them all, this binary is simply a frontend to the binaries above which presents them as subcommands.


## Don't use this tool/crate

I started this project to try and automate several tasks to auto tag
my MP3, deduplicate and organize them. But that ended up being a bunch
of work and after a little research I found the tools below, which
provide exactly what I wanted and is semi-automatic in the sense that
you will need to do some clicking and might need to configure a few
things as well. Totally worth giving the nearly perfect results,
though.

### So what then ?

I've been having much more success passing MP3 files through the following tools subsequently:

1. OneTagger ([https://onetagger.github.io/](https://onetagger.github.io/)): which uses Shazam to try and ID music, it fails sometimes but the next tool usually catches anything missing.
2. Picard by Musicbrainz ([https://picard.musicbrainz.org/](https://picard.musicbrainz.org/))
3. Dupsonic ([https://crates.io/crates/dupsonic](https://crates.io/crates/dupsonic)): deduplicates MP3 files by actually analyzing the audio
