Feature: Init code

  Scenario: Running init code
    When I init the database
    Then the function exists
