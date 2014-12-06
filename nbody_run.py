import subprocess, datetime

subprocess.call(["cargo", "build"]);

def single_run():
    p = subprocess.Popen(['./target/nbody'], stdout=subprocess.PIPE)
    out, err = p.communicate()

    fname = datetime.datetime.now().strftime("%Y-%b-%d__%H_%M_%S.txt")
    f = open("data/"+fname, 'wb')
    f.write(out)
    f.close()
    print("written to {}".format(fname))


def varying_initial_vel():
    dt = datetime.datetime.now().strftime("%Y-%b-%d__%H_%M_%S")


    vy = 0.3
    for i in range(0, 6):
        print(str(vy))
        p = subprocess.Popen(['./target/nbody', 'vel', '0', str(vy), '0'],
                             stdout=subprocess.PIPE)
        out, err = p.communicate()

        f = open("data/{}_{}.txt".format(dt, i), 'wb')
        f.write(out)
        f.close()
        vy += 0.1

    print("written to {}".format(dt))


varying_initial_vel()
