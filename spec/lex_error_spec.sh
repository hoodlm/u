#shellcheck shell=sh
Describe 'lexer'
  Parameters
    '~'
  End

  It "reports [$1] as not a valid token in the middle of a line"
    echo "1 + $1 STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 2
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "Unknown token"
    The line 2 of stderr should include $1
  End

  It "reports [$1] as not a valid token at the beginning of a line"
    echo "$1 + STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 2
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "Unknown token"
    The line 2 of stderr should include $1
  End

  It "reports [$1] as not a valid token at the end of a line"
    echo "5 + $1" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 2
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "Unknown token"
    The line 2 of stderr should include $1
  End
End
