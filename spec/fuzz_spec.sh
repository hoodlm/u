#shellcheck shell=sh
Describe 'random characters'
  It 'ascii stream from /dev/urandom does not crash the interpretter'
    head -c1000 /dev/urandom | od -vAn -ta | sed -E "s/\s+//g" >> $program
    When call $U_INTERPRETER $program
    The status should be failure
    The first line of stderr should include "Lexical analysis failed!"
  End
End
