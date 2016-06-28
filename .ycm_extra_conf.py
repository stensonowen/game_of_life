#trying to make a .ycm_extra_conf for a rust project
#I can't tell whether it works, ycm is still a little funky

import os, re
#import ycm_core

current_path = os.path.dirname(os.path.abspath(__file__))

flags = [
        "--crate-name life",
        "--crate-type bin",
        "-g",
        "--emit=dep-info,link",
        "-L dependency=" + current_path + "/target/debug",
        "-L dependency=" + current_path + "/target/debug/deps",
        ]


def FlagsForFile(filename):
    #add Cargo.toml dependencies
    flags_ = flags
    try:
        new_flags = []

        f = open('Cargo.toml', 'r')
        text = f.read()
        f.close()
        i = text.index("[dependencies]")
        text = text[i:]
        deps = re.findall("(\w+)\W*=", text)

        dep_path = current_path + "/target/debug/deps/"
        files = os.listdir(dep_path)
        for fn in files:
            for dep in deps:
                if "lib"+dep+"-" in fn:
                    s = "--extern %s=%s" % (dep, dep_path + fn)
                    new_flags.append(s)
        flags_ += new_flags
    except:
        pass

    return {
        'flags': flags_,
        'do_cache': True
    }
