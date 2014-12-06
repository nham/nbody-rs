import subprocess, datetime

subprocess.call(["cargo", "build"]);
p = subprocess.Popen(['./target/nbody'], stdout=subprocess.PIPE)
out, err = p.communicate()

fname = datetime.datetime.now().strftime("%Y-%b-%d__%H_%M_%S.txt")
f = open("data/"+fname, 'wb')
f.write(out)
f.close()
