#shellcheck shell=sh
Describe '+ operator'
  It 'can increment a positive integer'
    echo "5 + + + STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '8'
    The status should be success
  End

  It 'can increment a negative integer'
    echo "-2 + STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '-1'
    The status should be success
  End

  It 'can increment a negative integer to zero'
    echo "-2 + + STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '0'
    The status should be success
  End

  It 'can increment a negative integer to a positive number'
    echo "-2 + + + + + STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '3'
    The status should be success
  End

  It 'can increment a large integer'
    echo "562949953421312 + STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '562949953421313'
    The status should be success
  End

  It 'can be executed in a multi-line program'
    echo "1 + + + STDOUT" >> $program
    echo "10 + + + STDOUT" >> $program
    echo "100 + + + + STDOUT" >> $program
    When call $U_INTERPRETER $program
    The stdout lines should equal '3'
    The first line of stdout should eq '4'
    The second line of stdout should eq '13'
    The third line of stdout should eq '104'
    The status should be success
  End
End
