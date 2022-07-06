# As we can not change the directory of the current running
# shell we are using a function that will do it for us.
# You just need to set the path to the bmr binary and source
# the wrapper.

function bm () {
    # bmr is called by passing the bookmark files first. We will
    # use the wrapper to hide this to the user.
    # Help
    #     bmr <FILE> [OPTIONS] [NAME]
    # where
    #    ARGS:
    #        <FILE>    File that contains the bookmarks
    #        <NAME>    Name of the bookmark to use to change directory
    # options followed...
    BMR="../target/debug/bmr"
    BMK_FILE="./bm.yaml"
    ARGS="$@"

    if [ $# -eq 0 ] || [ "${ARGS:0:1}" = "-" ]
    then
	${BMR} ${BMK_FILE} ${ARGS}
    else
	cd $(${BMR} ${BMK_FILE} ${ARGS})
    fi
}
