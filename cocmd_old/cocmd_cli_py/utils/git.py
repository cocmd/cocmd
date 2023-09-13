import subprocess
import os

def clone(repo, local):
    p = subprocess.Popen(['git', 'clone', '--depth', '1', repo, local], stdout=subprocess.PIPE)
    return p.communicate()

def get_repo_name(remote_url):
    return os.path.splitext(os.path.basename(remote_url))[0]