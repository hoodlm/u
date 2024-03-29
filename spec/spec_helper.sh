spec_helper_configure() {
    ready_temp_dir
    set_u_binary
    before_each setup_tmp_program_file
}

set_u_binary() {
    if [ -n "$U_INTERPRETER" ]; then
        echo "U_INTERPRETER set: $U_INTERPRETER"
    else
        echo "U_INTERPRETER ENV variable not set, looking for an interpreter"
        U_INTERPRETER=$(command -v u || find_u)
        echo "Found: $U_INTERPRETER"
    fi
}

find_u() {
    find_result=$(find . -name "u" -type f | head -n1)
    if [ -z "$find_result" ]; then
        echo "Could not find 'u' executable"
        exit 1
    fi
    printf "%s" "$find_result"
}

ready_temp_dir() {
    program_dir=$(mktemp -d)
}

setup_tmp_program_file() {
    program=$(mktemp --tmpdir=$program_dir --suffix=.u)
}

