ffmpeg -f concat -safe 0 -i <(find "$(pwd)" -iname '*.mp3' -printf "file '%p'\n" | sort) -c copy output.mp3
