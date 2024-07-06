# Lyric Providers

The lyric providers below are used to fetch lyrics from players.

You can disable unnecessary lyric providers in the settings of the Lyrica widget.

## `mpris2_text`

If your player supports providing lyrics with timeline in asText of mpris2 metadata, then this lyric provider will be used.

## `file`

This lyrics provider reads lyrics from the music file's tags or LRC files when playing local music.

Supports the following file formats: mp3, flac, lrc.

## `feeluown_netease`

FeelUOwn is a local & online music player written in Python. This lyrics provider is available when using FeelUOwn to listen to songs on NetEase Cloud Music.

## `yesplaymusic`

YesPlayMusic is a third-party player for NetEase Cloud Music. Lyrica can get lyrics from the API provided by YesPlayMusic when using this player.

## `netease`

The `netease` lyrics provider finds the corresponding song and obtains the lyrics from NetEase Cloud Music. It can be used as a fallback option when other lyrics providers are unavailable.