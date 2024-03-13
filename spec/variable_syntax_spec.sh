#shellcheck shell=sh
Describe 'variables'
  Describe 'are immutable'
    It 'cannot assign value to the same variable twice in one line'
      echo '5 $zzz + $zzz STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be failure
      The line 1 of stderr should include 'Syntax analysis failed'
      The line 2 of stderr should include '$zzz'
    End

    It 'cannot assign to a variable twice on different lines'
      echo '5 $xyz;' >> $program
      echo '8 $xyz;' >> $program
      echo '$xyz STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be failure
      The line 1 of stderr should include 'Syntax analysis failed'
      The line 2 of stderr should include '$xyz'
    End
  End

  Describe 'must be assigned'
    It 'cannot use a variable that has not yet been assigned'
      echo '$x STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be failure
      The line 1 of stderr should include 'Syntax analysis failed'
      The line 2 of stderr should include '$x'
    End
  End

  Describe 'naming restrictions'
    It 'cannot contain hyphens'
      echo '5 + $my-variable;' >> $program
      echo '$my-variable STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be failure
      The stderr should include 'Lexical analysis failed'
    End

    It 'cannot contain numbers'
      echo '5 + $6;' >> $program
      echo '$6 STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be failure
      The stderr should include 'Lexical analysis failed'
    End
  End
End
