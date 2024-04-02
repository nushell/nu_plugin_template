#!/usr/bin/env nu
let tempdir = (mktemp --directory)
let template = $env.PWD

for command_is_simple in [Yes, No] {
    for multi_command in [Yes, No] {
        print ($"Testing with command_is_simple=($command_is_simple), " ++
            $"multi_command=($multi_command)")
        try {
            do --capture-errors {
                cd $tempdir
                (
                    ^cargo generate
                        --path $template
                        --force
                        --silent
                        --name nu_plugin_test_plugin
                        --define command_name="test command"
                        --define $"command_is_simple=($command_is_simple)"
                        --define $"multi_command=($multi_command)"
                        --define github_username=
                )
                do { cd nu_plugin_test_plugin; ^cargo test }
                rm -r nu_plugin_test_plugin
            }
        } catch { |err|
            print -e ($"Failed with command_is_simple=($command_is_simple), " ++
                $"multi_command=($multi_command)")
            rm -rf $tempdir
            $err.raw
        }
    }
}

rm -rf $tempdir
print "All tests passed."
