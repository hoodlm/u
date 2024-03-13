#shellcheck shell=sh
Describe 'variables'
  Describe 'with numerical values'
    It 'can store a value then print it out'
      echo '5 $y;' >> $program
      echo '$y STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq '5'
      The status should be success
    End

    It 'can store a value then have operators applied to it'
      echo '5 $five;' >> $program
      echo '$five + + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq '7'
      The status should be success
    End

    It 'can store a value after operators are applied to it'
      echo '5 + $y;' >> $program
      echo '$y + + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq '8'
      The status should be success
    End

    It 'can store intermediate values during a line'
      echo '5 $five + $six + $seven + $eight;' >> $program
      echo '$five STDOUT;' >> $program
      echo '$six STDOUT;' >> $program
      echo '$seven STDOUT;' >> $program
      echo '$eight STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout lines should eq 4
      The line 1 of stdout should eq '5'
      The line 2 of stdout should eq '6'
      The line 3 of stdout should eq '7'
      The line 4 of stdout should eq '8'
      The status should be success
    End
  End

  Describe 'with string values'
    It 'can store a string then print it out'
      echo '"hello" $greeting;' >> $program
      echo '$greeting STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq 'hello'
      The status should be success
    End
  End

  Describe 'naming restrictions'
    It 'can be snake_case'
      echo '5 + + $my_variable;' >> $program
      echo '$my_variable + + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq '9'
      The status should be success
    End

    It 'can be SNAKE_CASE'
      echo '5 + + $I_LIKE_TO_SHOUT;' >> $program
      echo '$I_LIKE_TO_SHOUT + + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq '9'
      The status should be success
    End

    It 'can be camelCase'
      echo '5 + + $myVariable;' >> $program
      echo '$myVariable + + STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The stdout should eq '9'
      The status should be success
    End
  End
End
