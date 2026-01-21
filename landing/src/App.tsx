import { Download, Eye, Brain, CheckSquare, Apple } from 'lucide-react'
import './App.css'

const GITHUB_RELEASE_URL = 'https://github.com/refcell/jarvis/releases/latest/download/Jarvis_0.1.1_universal.dmg'
const VERSION = '0.1.1'

function App() {
  return (
    <div className="container">
      <header className="header">
        <div className="logo">
          <Brain size={24} />
          <span>Jarvis</span>
        </div>
        <div className="version">v{VERSION}</div>
      </header>

      <main className="hero">
        <h1 className="headline">
          AI-powered task detection for your Mac
        </h1>
        <p className="subheadline">
          Jarvis watches your screen, analyzes context using LLMs, and automatically detects actionable tasks — displaying them in a prioritized dashboard.
        </p>

        <div className="cta-group">
          <a href={GITHUB_RELEASE_URL} className="download-button">
            <Apple size={20} />
            Download for macOS
            <Download size={16} />
          </a>
          <a href="https://github.com/refcell/jarvis" className="secondary-button">
            View on GitHub
          </a>
        </div>

        <p className="requirements">
          Requires macOS 13.0 or later · Apple Silicon
        </p>
      </main>

      <section className="features">
        <div className="feature">
          <Eye size={24} className="feature-icon" />
          <h3>Screen Monitoring</h3>
          <p>Captures screen context intelligently without storing sensitive data</p>
        </div>
        <div className="feature">
          <Brain size={24} className="feature-icon" />
          <h3>LLM Analysis</h3>
          <p>Uses Claude, OpenAI, Ollama, or CLI tools to understand your workflow</p>
        </div>
        <div className="feature">
          <CheckSquare size={24} className="feature-icon" />
          <h3>Task Detection</h3>
          <p>Automatically identifies actionable items and prioritizes them for you</p>
        </div>
      </section>

      <footer className="footer">
        <p>Built with Tauri, React, and Rust</p>
      </footer>
    </div>
  )
}

export default App
