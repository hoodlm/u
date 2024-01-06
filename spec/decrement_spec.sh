#shellcheck shell=sh
Describe '- operator'
  It 'can decrement a positive integer'
    echo "5 - STDOUT" >> $program
    When call u $program
    The stdout should eq '4'
    The status should be success
  End

  It 'can decrement a negative integer'
    echo "-2 - STDOUT" >> $program
    When call u $program
    The stdout should eq '-3'
    The status should be success
  End

  It 'can decrement a positive integer to zero'
    echo "2 - - STDOUT" >> $program
    When call u $program
    The stdout should eq '0'
    The status should be success
  End

  It 'can decrement a positive integer to a negative number'
    echo "2 - - - - - STDOUT" >> $program
    When call u $program
    The stdout should eq '-3'
    The status should be success
  End

  It 'can decrement a large integer'
    echo "562949953421312 - STDOUT" >> $program
    When call u $program
    The stdout should eq '562949953421311'
    The status should be success
  End

  It 'can be executed in a multi-line program'
    echo "1 - - - STDOUT" >> $program
    echo "10 - - - STDOUT" >> $program
    echo "100 - - - - STDOUT" >> $program
    When call u $program
    The stdout lines should equal '3'
    The first line of stdout should eq '-2'
    The second line of stdout should eq '7'
    The third line of stdout should eq '96'
    The status should be success
  End
End
