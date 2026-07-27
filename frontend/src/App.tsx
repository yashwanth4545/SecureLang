import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { Shield, BookOpen, Download, Terminal, Code, Home, User, Settings } from 'lucide-react';
import { motion } from 'framer-motion';

// Mock Pages (In a real app, these would be separate files)
const Landing = () => (
  <div className="min-h-[80vh] flex flex-col items-center justify-center text-center px-4">
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.8 }}
    >
      <Shield className="w-24 h-24 text-secureAccent mx-auto mb-8" />
      <h1 className="text-6xl font-bold mb-6 tracking-tight">
        Build with <span className="text-secureAccent">Absolute</span> Security.
      </h1>
      <p className="text-xl text-gray-400 max-w-2xl mx-auto mb-10">
        The first enterprise programming language with built-in authentication, RBAC, and memory safety at the core.
      </p>
      <div className="flex gap-4 justify-center">
        <Link to="/docs" className="bg-secureAccent text-secureDark px-8 py-3 rounded-lg font-bold hover:bg-white transition-colors">
          Read Documentation
        </Link>
        <Link to="/downloads" className="glass-panel px-8 py-3 rounded-lg font-bold hover:bg-white/10 transition-colors">
          Download SDK
        </Link>
      </div>
    </motion.div>
  </div>
);

const Docs = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Documentation</h2><p>Welcome to SecureLang docs.</p></div>;
const Downloads = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Downloads</h2><p>Get the latest SecureLang compiler and CLI.</p></div>;
const Playground = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Playground</h2><p>Try SecureLang in your browser.</p></div>;
const Examples = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Examples</h2><p>Code snippets and use cases.</p></div>;
const Tutorials = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Tutorials</h2><p>Step by step tutorials.</p></div>;
const ApiDocs = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">API Docs</h2><p>Standard library references.</p></div>;
const Packages = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Package Registry</h2><p>Search for SecureLang packages.</p></div>;
const Blog = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Blog</h2><p>Latest updates and security news.</p></div>;
const Dashboard = () => <div className="p-8"><h2 className="text-3xl font-bold mb-4">Dashboard</h2><p>Your secure applications and tokens.</p></div>;

const App = () => {
  return (
    <Router>
      <div className="min-h-screen flex flex-col">
        <header className="border-b border-white/10 bg-secureDark/80 backdrop-blur-md sticky top-0 z-50">
          <div className="max-w-7xl mx-auto px-4 h-16 flex items-center justify-between">
            <Link to="/" className="flex items-center gap-2 text-xl font-bold">
              <Shield className="w-6 h-6 text-secureAccent" />
              SecureLang
            </Link>
            <nav className="flex gap-4 text-sm font-medium text-gray-300">
              <Link to="/docs" className="hover:text-white transition-colors">Docs</Link>
              <Link to="/downloads" className="hover:text-white transition-colors">Downloads</Link>
              <Link to="/playground" className="hover:text-white transition-colors">Playground</Link>
              <Link to="/examples" className="hover:text-white transition-colors">Examples</Link>
              <Link to="/packages" className="hover:text-white transition-colors">Registry</Link>
              <Link to="/blog" className="hover:text-white transition-colors">Blog</Link>
            </nav>
            <div className="flex gap-4">
              <Link to="/login" className="text-sm font-medium hover:text-white transition-colors">Login</Link>
              <Link to="/dashboard" className="text-sm font-medium text-secureAccent hover:text-white transition-colors">Dashboard</Link>
            </div>
          </div>
        </header>

        <main className="flex-grow max-w-7xl mx-auto w-full">
          <Routes>
            <Route path="/" element={<Landing />} />
            <Route path="/docs" element={<Docs />} />
            <Route path="/downloads" element={<Downloads />} />
            <Route path="/playground" element={<Playground />} />
            <Route path="/examples" element={<Examples />} />
            <Route path="/tutorials" element={<Tutorials />} />
            <Route path="/api-docs" element={<ApiDocs />} />
            <Route path="/packages" element={<Packages />} />
            <Route path="/blog" element={<Blog />} />
            <Route path="/dashboard" element={<Dashboard />} />
          </Routes>
        </main>

        <footer className="border-t border-white/10 py-8 text-center text-gray-500 text-sm">
          <p>&copy; {new Date().getFullYear()} SecureLang Foundation. All rights reserved.</p>
        </footer>
      </div>
    </Router>
  );
};

export default App;
