import os
import subprocess
import time
import shlex

print('1 hours: 3600 seconds, 2 hours: 7200 seconds, 3 hours: 10800 seconds, 4 hours: 14400 seconds, 5 hours: 18000 seconds.')
time.sleep(0.5)
_seconds = input('How many seconds to split the files? ')

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
        proc = subprocess.run(shlex.split('ffmpeg -i "'+_dest+'" -f segment -segment_time '+_seconds+' -c copy "'+prepath+i+'/Part%03d.mp3"'), stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        out = proc.stdout
        print("Done with " + i)
        os.remove(_dest)


    running = 0
