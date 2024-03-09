#shellcheck shell=sh
Describe 'syntax analyzer errors'
  It "reports error if line starts with an operator"
    echo "+ 1 STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The status should be failure
    The stdout should be blank
    The stderr should not be blank
    The line 1 of stderr should include "Syntax analysis failed"
    The line 2 of stderr should include "Unexpected token:"
    The line 2 of stderr should include "Plus"
  End

  It "reports error if line starts with two source tokens"
    echo "1 1 + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The status should be failure
    The stdout should be blank
    The lines of stderr should eq 2
    The line 1 of stderr should include "Syntax analysis failed"
  End

  It "reports error if line starts has a source token in the middle"
    echo "1 + 1 + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The status should be failure
    The stdout should be blank
    The lines of stderr should eq 2
    The line 1 of stderr should include "Syntax analysis failed"
  End

  It "reports error if line is incomplete"
    echo "1 +" >> $program
    When call $U_INTERPRETER $program
    The status should be failure
    The stdout should be blank
    The lines of stderr should eq 2
    The line 1 of stderr should include "Syntax analysis failed"
    The line 2 of stderr should include "Expected more tokens before end of line"
  End

  It "reports error if a multi-line program is incomplete"
    echo "1 + + STDOUT;" >> $program
    echo "1 +" >> $program
    When call $U_INTERPRETER $program
    The status should be failure
    The stdout should be blank
    The lines of stderr should eq 2
    The line 1 of stderr should include "Syntax analysis failed"
    The line 2 of stderr should include "Expected more tokens before end of line"
  End
End
