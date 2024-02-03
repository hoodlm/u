#shellcheck shell=sh
Describe 'lexer single error report'
  Parameters
    '~'
    '#foo'
    '&asdf'
    '@@@@'
    '/a'
    '$a'
    '*Z'
    '$1.50'
    '$20'
    '1,000'
    '1_000'
    '1.23E5'
    'Ω'
    '👾'
  End

  It "reports [$1] as not a valid token in the middle of a line"
    echo "1 + $1 STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 2
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "Unknown token"
    The line 2 of stderr should include $1
  End

  It "reports [$1] as not a valid token at the beginning of a line"
    echo "$1 + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 2
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "Unknown token"
    The line 2 of stderr should include $1
  End

  It "reports [$1] as not a valid token at the end of a line"
    echo "5 + $1;" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 2
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "Unknown token"
    The line 2 of stderr should include $1
  End
End

Describe 'lexer multi error report'
  It "Reports errors on mulitple lines"
    echo "5 + + STDOUT;" >> $program
    echo "@@@@ + + STDOUT;" >> $program
    echo "6 + + STDOUT;" >> $program
    echo "##### + + STDOUT;" >> $program
    echo "//// + + STDOUT;" >> $program
    echo "7 + + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 4
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "@@@@"
    The line 3 of stderr should include "#####"
    The line 4 of stderr should include "////"
  End
End

Describe 'characters'
  It "must be single-quoted"
    echo "A + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should be blank
    The status should be failure
    The lines of stderr should eq 2
    The line 1 of stderr should eq "Lexical analysis failed!"
    The line 2 of stderr should include "Unknown token: A"
  End
End
