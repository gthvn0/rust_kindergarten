# As we can not change the directory of the current running
# shell we are using a function that will do it for us.
# You just need to set the path to the bmr binary and source
# the wrapper.
function bm () {

    arg="$@"
    bmr="/path/to/bmr"

    if [ "${arg:0:1}" = "-" ]
    then
	${bmr} ${arg}
    else
	cd $(${bmr} ${arg})
    fi
}
