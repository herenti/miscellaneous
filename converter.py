import os
import subprocess
import time
import shlex

todir = os.listdir(os.getcwd())
mergefile = open('convert.sh', 'r').read()
#extention = 'mp3'


prepath = os.getcwd()+'/'

to_process = {}

def invoke_at(path: str):
    def parameterized(func):
        def wrapper(*args, **kwargs):
            cwd = os.getcwd()
            os.chdir(path)

            try:
                ret = func(*args, **kwargs)
            finally:
                os.chdir(cwd)

            return ret

        return wrapper

    return parameterized

for i in todir:
    try:
        a = os.listdir(prepath+i)
        to_process[i] = a
    except:
        continue

for i in to_process:
    _file = prepath+i

    @invoke_at(r'%s' % _file)
    def start_the_tool():
        print(os.getcwd())
        f = open(_file+'/convert.sh', 'w')
        f.write(mergefile)
        f.close()
        proc = subprocess.run(shlex.split('sh convert.sh'), stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        out = proc.stdout
        print(out)
        os.remove(_file+'/merge.sh')
        for x in to_process[i]:
            os.remove(x)

    start_the_tool()

