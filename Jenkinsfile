node('master') {
    try {
        stage 'Checkout'
            checkout scm
        stage 'Version Check'
            sh 'rustc --version'
            sh 'cargo --version'
        stage 'Build'
            sh 'cargo build'
        stage 'test'
            sh 'cargo test'
        currentBuild.result = "SUCCESS"
    } catch (err) {
        currentBuild.result = "FAILURE"
        throw err
    }
}
