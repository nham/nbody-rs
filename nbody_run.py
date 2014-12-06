import subprocess, datetime

print("about to run cargo")
subprocess.call(["cargo", "build"]);
print("now opening a pipe to nbody")
p = subprocess.Popen(['./target/nbody'], stdout=subprocess.PIPE)
out, err = p.communicate()

fname = datetime.datetime.now().strftime("%Y-%b-%d__%H_%M.txt")
f = open("data/"+fname, 'wb')
f.write(out)
f.close()
