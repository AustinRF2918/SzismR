import os
import sys
import errno
import shutil

#Stolen from stack overflow, google it
def copytree(src, dst, symlinks=False, ignore=None):
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            shutil.copytree(s, d, symlinks, ignore)
        else:
            shutil.copy2(s, d)

scriptFolder = os.path.dirname(os.path.realpath(sys.argv[0]))

copytree(scriptFolder + "/" + "templates" + "/" + "web-template", os.getcwd());
