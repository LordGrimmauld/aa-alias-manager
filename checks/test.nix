{
  extraPythonPackages = p: [ p.regex ];
  testScript = # python
    ''
      print("Starting VM test...")

      import re
      machine.wait_for_unit("default.target")
      # result = machine.succeed("aa-status")

      result = machine.succeed("journalctl -u aa-alias-setup -b 0")
      if "aa-alias-setup.service: Deactivated successfully." not in result: 
        raise Exception("aa-alias-setup deactivated with non-zero exit code")


      result = machine.succeed("journalctl -u apparmor -b 0")
      if "Finished Load AppArmor policies." not in result: 
        raise Exception("AppArmor did not finish loading policies.")

      machine.succeed("ls /run/aliases.d")

      result = machine.succeed('sleep 3 & >&2 && aa-status')
      if re.search(r"[1-9][0-9]* profiles are in complain mode", result) is None:
        print(result)
        raise Exception("Expected profiles in complain mode, none were found.")

      if re.search(r"[1-9][0-9]* processes are in complain mode", result) is None:
        print(result)
        raise Exception("Expected processes in complain mode, none were found.")
    '';
}
