import os
import subprocess
import shlex

todir = os.listdir(os.getcwd())

extention = 'mp3'

running = 1

prepath = os.getcwd()+'/'

processing = 0

_dict = {}

todir = [_dict.update({i.replace('.'+i.split('.')[-1],''): i}) for i in todir if i.split('.')[-1] == extention]

todir = _dict

while running == 1:

    for i in todir:
        processing = 1
        _file = todir[i]
        _dest = prepath+i+'/'+_file
        os.mkdir(prepath+i)
        os.rename(_file, _dest)
        proc = subprocess.run(shlex.split('ffmpeg -i "'+_dest+'" -f segment -segment_time 14400 -c copy "'+prepath+i+'/Part%03d.mp3"'), stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        out = proc.stdout
        print("Done with " + i)
        os.remove(_dest)


    running = 0
