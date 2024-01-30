#shellcheck shell=sh
Describe 'floating point numbers'
  It 'can be incremented'
    echo "1.5 + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '2.5'
    The status should be success
  End

  It 'can be decremented'
    echo "1.5 - STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '0.5'
    The status should be success
  End

  It 'can be used on multiple line programs'
    echo "1.5 - STDOUT;" >> $program
    echo "1.5 + STDOUT;" >> $program
    echo "1.5 + + + + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout lines should equal '3'
    The first line of stdout should eq '0.5'
    The second line of stdout should eq '2.5'
    The third line of stdout should eq '5.5'
    The status should be success
  End
End

Describe 'negative floating point numbers'
  It 'can be incremented'
    echo "-100.123 + + STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '-98.123'
    The status should be success
  End

  It 'can be decremented'
    echo "-3.25 - STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '-4.25'
    The status should be success
  End
End
