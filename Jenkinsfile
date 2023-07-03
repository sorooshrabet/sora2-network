@Library('jenkins-library@feature/dops-2395/rust_library') _

def pipeline = new org.rust.AppPipeline(steps: this,
      assignReviewers: true,
      disableSecretScanner: false,
      substrate: true,
      secretScannerExclusion: '.*Cargo.toml\$|.*pr.sh\$',
      pushTags: ['develop': 'dev', 'master': 'latest'],
      envImageName: 'docker.soramitsu.co.jp/sora2/env:sub4',
      appImageName: 'docker.soramitsu.co.jp/sora2/substrate',
      benchmarkingBase: 'develop',
      codeCoverageCommand: './housekeeping/coverage.sh',
      coberturaReportFile: 'cobertura_report',
      cargoDoc: true,
      prStatusNotif: true,
      smartContractScanner: true,
      buildTestCmds: ['housekeeping/build.sh'],
      buildArtifacts: 'framenode_runtime.compact.wasm, framenode_runtime.compact.compressed.wasm, subwasm_report.json, pallet_list.txt',
      codeCoverageCommand: true
      )
pipeline.runPipeline()
