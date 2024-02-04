#shellcheck shell=sh
Describe '{} repeater'
  It 'can be used to repeat the increment operator'
    echo "5 {3} + STDOUT;" >> $program
    When call $U_INTERPRETER $program

    # TODO: Not yet implemented, still a syntax error
    # The stdout should eq '8'
    # The status should be success

    The status should be failure
    The stderr should include "Syntax analysis failed!"
  End

  It 'can be used to repeat the decrement operator'
    echo "10 {5} - STDOUT;" >> $program
    When call $U_INTERPRETER $program

    # TODO: Not yet implemented, still a syntax error
    # The stdout should eq '5'
    # The status should be success

    The status should be failure
    The stderr should include "Syntax analysis failed!"
  End

  It 'can be used to repeat the STDOUT operator'
    echo '"Hello world" {2} STDOUT;' >> $program
    When call $U_INTERPRETER $program

    # TODO: Not yet implemented, still a syntax error
    # The status should be success
    # The first line of stdout should eq 'HELLO WORLD'
    # The second line of stdout should eq 'HELLO WORLD'

    The status should be failure
    The stderr should include "Syntax analysis failed!"
  End
End
