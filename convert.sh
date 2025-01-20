for i in *.mp3;
  do name=`echo "$i" | cut -d'.' -f1`
  echo "$name"
  ffmpeg -i "$i" -b:a 64k -vn "_${name}.mp3"
done
