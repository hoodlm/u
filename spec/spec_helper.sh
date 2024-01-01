spec_helper_configure() {
    before_all ensure_u_on_path
    before_each setup_tmp_program_file
}

ensure_u_on_path() {
    command -v u || find_u
}

find_u() {
    echo "u is not on PATH, looking for it..."
    find_result=$(find . -name "u" -type f | head -n1)
    if [ -z "$find_result" ]; then
        echo "Could not find 'u' executable"
        exit 1
    fi
    printf "Found interpreter: %s\n" "$find_result"
    u_dir=$(dirname "${find_result}")
    printf "Adding '%s' to PATH\n" "$u_dir"

    PATH="$PATH:$u_dir"
}

setup_tmp_program_file() {
    program=$(mktemp --suffix=.u)
}

