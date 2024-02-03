#shellcheck shell=sh
Describe 'STDOUT operator'
  It 'can occur in the middle of a line'
    echo "5 + STDOUT + +;" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '6'
    The status should be success
  End

  It 'can occur without other operators'
    echo "1 STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '1'
    The status should be success
  End

  It 'can occur multiple times per line'
    echo "0 + STDOUT + STDOUT + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout lines should equal '3'
    The first line of stdout should eq '1'
    The second line of stdout should eq '2'
    The third line of stdout should eq '3'
    The status should be success
  End
End
