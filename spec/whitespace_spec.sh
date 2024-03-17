#shellcheck shell=sh
Describe 'Whitespace'
  It 'newlines can separate tokens'
    echo "5" >> $program
    echo "+" >> $program
    echo "STDOUT" >> $program
    echo ";" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '6'
    The status should be success
  End

  It 'Empty lines can separate tokens'
    echo "5" >> $program
    echo "\n\n\n" >> $program
    echo "-" >> $program
    echo "\n\n\n" >> $program
    echo "STDOUT" >> $program
    echo "\n\n\n" >> $program
    echo ";" >> $program
    When call $U_INTERPRETER $program
    The stdout should eq '4'
    The status should be success
  End

  It 'Multi-statement programs can span multiple lines'
    echo "5 +" >> $program
    echo "STDOUT;" >> $program
    echo "10 -" >> $program
    echo "STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The status should be success
    The first line of stdout should eq '6'
    The second line of stdout should eq '9'
  End

  It 'Operators can occur without whitespace'
    echo "5 +-+-+STDOUT+-+-+STDOUT;" >> $program
    When call $U_INTERPRETER $program
    The status should be success
    The first line of stdout should eq '6'
    The second line of stdout should eq '7'
  End

  Describe 'No whitespace is required at all'
    It 'after integers'
      echo "5+STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The first line of stdout should eq '6'
    End

    It 'after floats'
      echo "-5.5-STDOUT;" >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The first line of stdout should eq '-6.5'
    End

    It 'after a string'
      echo '"hello world"+STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The first line of stdout should eq 'ifmmp xpsme'
    End

    It 'between operators'
      echo '1+STDOUTSTDOUT++STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The first line of stdout should eq '2'
      The second line of stdout should eq '2'
      The third line of stdout should eq '4'
    End

    It 'between commands'
      echo '1+STDOUT;10-STDOUT;' >> $program
      When call $U_INTERPRETER $program
      The status should be success
      The first line of stdout should eq '2'
      The second line of stdout should eq '9'
    End
  End
End
