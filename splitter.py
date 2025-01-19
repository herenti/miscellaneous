import os
import subprocess
from pathlib import Path
import stat
import shlex

todir = os.listdir(os.getcwd())

extention = 'mp3'

running = 1

prepath = os.getcwd()+'/'

processing = 0

_script = open('split.sh', 'r').read()

_dict = {}

todir = [_dict.update({i.replace('.'+i.split('.')[-1],''): i}) for i in todir if i.split('.')[-1] == extention]

todir = _dict

while running == 1:

    for i in todir:
        def _process():
            processing = 1
            _file = todir[i]
            _dest = prepath+i+'/'+_file
            os.mkdir(prepath+i)
            f = open(prepath+i+"/split.sh", "w")
            f.write(_script)
            f.close()
            f = Path(prepath+i+"/split.sh")
            f.chmod(f.stat().st_mode | stat.S_IEXEC)
            os.rename(_file, _dest)
            os.rename(_dest, prepath+i+'/output.mp3')
            proc = subprocess.run(shlex.split('ffmpeg -i "'+prepath+i+'/output.mp3" -f segment -segment_time 14400 -c copy "'+prepath+i+'/Part%03d.mp3"'), stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            out = proc.stdout

            print(out)
            processing = 0

        if processing == 0:
            _process()
        else:
            pass

    running = 0
