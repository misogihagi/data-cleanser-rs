// The exported code uses Tailwind CSS. Install Tailwind CSS in your dev environment to ensure all styles work.
import React, { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
const App: React.FC = () => {
  const [isScrolled, setIsScrolled] = useState(false);
  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 50);
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);
  const portfolioItems = [
    {
      id: 1,
      title: "Digital Art Collection",
      category: "NFT",
      description:
        "Exclusive digital artwork featuring vibrant colors and abstract compositions",
      image:
        "https://readdy.ai/api/search-image?query=stunning%20digital%20abstract%20art%20with%20vibrant%20neon%20colors%20and%20geometric%20patterns%20on%20dark%20background%2C%20modern%20artistic%20composition%20with%20glowing%20elements%2C%20high%20quality%20digital%20artwork%2C%20contemporary%20style%20with%20electric%20blue%20and%20purple%20accents&width=400&height=300&seq=portfolio1&orientation=landscape",
    },
    {
      id: 2,
      title: "Brand Identity Design",
      category: "Branding",
      description:
        "Complete brand identity package with logo, colors, and typography",
      image:
        "https://readdy.ai/api/search-image?query=modern%20brand%20identity%20design%20showcase%20with%20elegant%20logo%20concepts%2C%20typography%20samples%2C%20and%20color%20palettes%20on%20dark%20background%2C%20professional%20branding%20materials%20with%20clean%20minimalist%20aesthetic%2C%20corporate%20design%20elements&width=400&height=300&seq=portfolio2&orientation=landscape",
    },
    {
      id: 3,
      title: "Mobile App Interface",
      category: "UI/UX",
      description:
        "Intuitive mobile application design with seamless user experience",
      image:
        "https://readdy.ai/api/search-image?query=sleek%20mobile%20app%20interface%20design%20mockup%20on%20dark%20background%2C%20modern%20smartphone%20screen%20showing%20elegant%20user%20interface%2C%20clean%20app%20design%20with%20purple%20and%20blue%20accents%2C%20professional%20mobile%20UI%20showcase&width=400&height=300&seq=portfolio3&orientation=landscape",
    },
    {
      id: 4,
      title: "3D Visualization",
      category: "3D Design",
      description:
        "Photorealistic 3D renders for architectural and product visualization",
      image:
        "https://readdy.ai/api/search-image?query=impressive%203D%20architectural%20visualization%20with%20modern%20building%20design%20on%20dark%20background%2C%20photorealistic%20rendering%20with%20dramatic%20lighting%2C%20contemporary%20architecture%20with%20glass%20and%20steel%20elements%2C%20professional%203D%20artwork&width=400&height=300&seq=portfolio4&orientation=landscape",
    },
    {
      id: 5,
      title: "Web Development",
      category: "Development",
      description: "Responsive websites built with cutting-edge technologies",
      image:
        "https://readdy.ai/api/search-image?query=modern%20web%20development%20showcase%20with%20clean%20code%20editor%20interface%20on%20dark%20background%2C%20responsive%20website%20design%20mockups%2C%20professional%20web%20development%20workspace%20with%20multiple%20screens%2C%20contemporary%20tech%20aesthetic&width=400&height=300&seq=portfolio5&orientation=landscape",
    },
    {
      id: 6,
      title: "Motion Graphics",
      category: "Animation",
      description: "Dynamic animations and motion graphics for digital media",
      image:
        "https://readdy.ai/api/search-image?query=dynamic%20motion%20graphics%20design%20with%20flowing%20abstract%20shapes%20and%20particles%20on%20dark%20background%2C%20colorful%20animation%20elements%20with%20neon%20trails%2C%20modern%20digital%20art%20with%20movement%20effects%2C%20vibrant%20motion%20design&width=400&height=300&seq=portfolio6&orientation=landscape",
    },
  ];
  const categories = [
    "All",
    "NFT",
    "Branding",
    "UI/UX",
    "3D Design",
    "Development",
    "Animation",
  ];
  const [activeCategory, setActiveCategory] = useState("All");
  const filteredItems =
    activeCategory === "All"
      ? portfolioItems
      : portfolioItems.filter((item) => item.category === activeCategory);
  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Header */}
      <header
        className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
          isScrolled ? "bg-gray-900/95 backdrop-blur-sm" : "bg-transparent"
        }`}
      >
        <div className="max-w-7xl mx-auto px-6 py-4">
          <nav className="flex items-center justify-between">
            <div className="text-2xl font-bold text-white">
              <i className="fas fa-cube mr-2 text-purple-400"></i>
              CreativeStudio
            </div>
            <div className="hidden md:flex items-center space-x-8">
              <a
                href="#home"
                className="text-gray-300 hover:text-white transition-colors cursor-pointer"
              >
                Home
              </a>
              <a
                href="#portfolio"
                className="text-gray-300 hover:text-white transition-colors cursor-pointer"
              >
                Portfolio
              </a>
              <a
                href="#about"
                className="text-gray-300 hover:text-white transition-colors cursor-pointer"
              >
                About
              </a>
              <a
                href="#services"
                className="text-gray-300 hover:text-white transition-colors cursor-pointer"
              >
                Services
              </a>
              <a
                href="#contact"
                className="text-gray-300 hover:text-white transition-colors cursor-pointer"
              >
                Contact
              </a>
            </div>
            <Button className="!rounded-button whitespace-nowrap bg-purple-600 hover:bg-purple-700 text-white cursor-pointer">
              Get Started
            </Button>
          </nav>
        </div>
      </header>
      {/* Hero Section */}
      <section
        id="home"
        className="relative min-h-screen flex items-center justify-center overflow-hidden"
      >
        <div
          className="absolute inset-0 bg-cover bg-center bg-no-repeat"
          style={{
            backgroundImage: `url('https://readdy.ai/api/search-image?query=futuristic%20dark%20digital%20workspace%20with%20holographic%20elements%20and%20neon%20purple%20lighting%2C%20modern%20tech%20environment%20with%20floating%20geometric%20shapes%2C%20cyberpunk%20aesthetic%20with%20glowing%20particles%20and%20abstract%20digital%20patterns%2C%20dark%20background%20perfect%20for%20text%20overlay&width=1440&height=800&seq=hero1&orientation=landscape')`,
          }}
        >
          <div className="absolute inset-0 bg-gradient-to-r from-gray-900/90 via-gray-900/70 to-transparent"></div>
        </div>
        <div className="relative z-10 max-w-7xl mx-auto px-6 py-20">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
            <div className="space-y-8">
              <h1 className="text-5xl lg:text-7xl font-bold leading-tight">
                Creative
                <span className="block text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-400">
                  Excellence
                </span>
                Redefined
              </h1>
              <p className="text-xl text-gray-300 leading-relaxed max-w-lg">
                We craft extraordinary digital experiences that push boundaries
                and inspire innovation. From concept to creation, we bring your
                vision to life with cutting-edge design and technology.
              </p>
              <div className="flex flex-col sm:flex-row gap-4">
                <Button className="!rounded-button whitespace-nowrap bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white px-8 py-3 text-lg cursor-pointer">
                  <i className="fas fa-rocket mr-2"></i>
                  View Portfolio
                </Button>
                <Button className="!rounded-button whitespace-nowrap bg-transparent border-2 border-gray-600 hover:border-purple-400 text-white px-8 py-3 text-lg cursor-pointer">
                  <i className="fas fa-play mr-2"></i>
                  Watch Demo
                </Button>
              </div>
            </div>
          </div>
        </div>
        {/* Scroll Indicator */}
        <div className="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
          <i className="fas fa-chevron-down text-2xl text-gray-400"></i>
        </div>
      </section>
      {/* Portfolio Section */}
      <section id="portfolio" className="py-20 bg-gray-800">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold mb-6">
              Featured <span className="text-purple-400">Portfolio</span>
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              Discover our latest projects showcasing innovative design
              solutions and cutting-edge technology implementations across
              various industries and creative domains.
            </p>
          </div>
          {/* Category Filter */}
          <div className="flex flex-wrap justify-center gap-4 mb-12">
            {categories.map((category) => (
              <Button
                key={category}
                onClick={() => setActiveCategory(category)}
                className={`!rounded-button whitespace-nowrap cursor-pointer ${
                  activeCategory === category
                    ? "bg-purple-600 hover:bg-purple-700 text-white"
                    : "bg-gray-700 hover:bg-gray-600 text-gray-300"
                }`}
              >
                {category}
              </Button>
            ))}
          </div>
          {/* Portfolio Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {filteredItems.map((item) => (
              <Card
                key={item.id}
                className="bg-gray-700 border-gray-600 hover:bg-gray-600 transition-all duration-300 hover:scale-105 hover:shadow-2xl hover:shadow-purple-500/20 cursor-pointer overflow-hidden"
              >
                <div className="relative overflow-hidden">
                  <img
                    src={item.image}
                    alt={item.title}
                    className="w-full h-48 object-cover object-top transition-transform duration-300 hover:scale-110"
                  />
                  <div className="absolute inset-0 bg-gradient-to-t from-gray-900/80 to-transparent opacity-0 hover:opacity-100 transition-opacity duration-300 flex items-end p-4">
                    <Button className="!rounded-button whitespace-nowrap bg-purple-600 hover:bg-purple-700 text-white cursor-pointer">
                      <i className="fas fa-external-link-alt mr-2"></i>
                      View Project
                    </Button>
                  </div>
                </div>
                <CardContent className="p-6">
                  <a
                    href="https://readdy.ai/home/f93329ae-41e6-46f7-bd2e-a773d56e0fc4/51b4daa8-f6b2-483b-8a2b-6c1a67ab86de"
                    data-readdy="true"
                    className="block"
                  >
                    <div className="flex items-center justify-between mb-3">
                      <span className="px-3 py-1 bg-purple-600/20 text-purple-300 text-sm rounded-full">
                        {item.category}
                      </span>
                      <i className="fas fa-heart text-gray-500 hover:text-red-400 cursor-pointer transition-colors"></i>
                    </div>
                    <h3 className="text-xl font-semibold mb-2 text-white">
                      {item.title}
                    </h3>
                    <p className="text-gray-400 text-sm leading-relaxed">
                      {item.description}
                    </p>
                  </a>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </section>
      {/* About Section */}
      <section id="about" className="py-20 bg-gray-900">
        <div className="max-w-7xl mx-auto px-6">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
            <div className="space-y-8">
              <h2 className="text-4xl lg:text-5xl font-bold">
                About <span className="text-purple-400">Our Studio</span>
              </h2>
              <p className="text-lg text-gray-300 leading-relaxed">
                We are a collective of passionate designers, developers, and
                creative minds dedicated to pushing the boundaries of digital
                innovation. With over a decade of experience, we've helped
                brands transform their vision into reality through exceptional
                design and technology.
              </p>
              <div className="grid grid-cols-2 gap-8">
                <div className="text-center">
                  <div className="text-3xl font-bold text-purple-400 mb-2">
                    150+
                  </div>
                  <div className="text-gray-400">Projects Completed</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-purple-400 mb-2">
                    50+
                  </div>
                  <div className="text-gray-400">Happy Clients</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-purple-400 mb-2">
                    10+
                  </div>
                  <div className="text-gray-400">Years Experience</div>
                </div>
                <div className="text-center">
                  <div className="text-3xl font-bold text-purple-400 mb-2">
                    25+
                  </div>
                  <div className="text-gray-400">Awards Won</div>
                </div>
              </div>
            </div>
            <div className="relative">
              <img
                src="https://readdy.ai/api/search-image?query=modern%20creative%20team%20working%20in%20futuristic%20dark%20office%20space%20with%20multiple%20monitors%20and%20holographic%20displays%2C%20professional%20designers%20and%20developers%20collaborating%20on%20innovative%20projects%2C%20contemporary%20workspace%20with%20purple%20and%20blue%20ambient%20lighting%2C%20high-tech%20creative%20environment&width=600&height=500&seq=about1&orientation=landscape"
                alt="About Us"
                className="w-full h-full object-cover object-top rounded-lg"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-purple-600/20 to-transparent rounded-lg"></div>
            </div>
          </div>
        </div>
      </section>
      {/* Services Section */}
      <section id="services" className="py-20 bg-gray-800">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold mb-6">
              Our <span className="text-purple-400">Services</span>
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              We offer comprehensive creative solutions tailored to meet your
              unique needs and exceed your expectations in the digital
              landscape.
            </p>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {[
              {
                icon: "fas fa-palette",
                title: "Brand Design",
                description:
                  "Complete brand identity creation including logo design, color schemes, and visual guidelines.",
              },
              {
                icon: "fas fa-code",
                title: "Web Development",
                description:
                  "Custom websites and web applications built with modern technologies and best practices.",
              },
              {
                icon: "fas fa-mobile-alt",
                title: "Mobile Apps",
                description:
                  "Native and cross-platform mobile applications with intuitive user interfaces.",
              },
              {
                icon: "fas fa-cube",
                title: "3D Modeling",
                description:
                  "Photorealistic 3D renders and animations for products, architecture, and visualization.",
              },
              {
                icon: "fas fa-video",
                title: "Motion Graphics",
                description:
                  "Engaging animations and motion graphics for marketing, explainer videos, and presentations.",
              },
              {
                icon: "fas fa-chart-line",
                title: "Digital Strategy",
                description:
                  "Comprehensive digital marketing strategies to enhance your online presence and growth.",
              },
            ].map((service, index) => (
              <Card
                key={index}
                className="bg-gray-700 border-gray-600 hover:bg-gray-600 transition-all duration-300 hover:scale-105 cursor-pointer group"
              >
                <CardContent className="p-8 text-center">
                  <div className="w-16 h-16 bg-gradient-to-r from-purple-600 to-pink-600 rounded-full flex items-center justify-center mx-auto mb-6 group-hover:scale-110 transition-transform duration-300">
                    <i className={`${service.icon} text-2xl text-white`}></i>
                  </div>
                  <h3 className="text-xl font-semibold mb-4 text-white">
                    {service.title}
                  </h3>
                  <p className="text-gray-400 leading-relaxed">
                    {service.description}
                  </p>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </section>
      {/* Contact Section */}
      <section id="contact" className="py-20 bg-gray-900">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-16">
            <h2 className="text-4xl lg:text-5xl font-bold mb-6">
              Let's <span className="text-purple-400">Connect</span>
            </h2>
            <p className="text-xl text-gray-300 max-w-3xl mx-auto">
              Ready to bring your vision to life? Get in touch with us and let's
              discuss how we can help you achieve your creative goals.
            </p>
          </div>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-16">
            <div className="space-y-8">
              <div className="flex items-start space-x-4">
                <div className="w-12 h-12 bg-purple-600 rounded-full flex items-center justify-center flex-shrink-0">
                  <i className="fas fa-map-marker-alt text-white"></i>
                </div>
                <div>
                  <h3 className="text-xl font-semibold mb-2 text-white">
                    Visit Our Studio
                  </h3>
                  <p className="text-gray-400">
                    123 Creative Street, Design District
                    <br />
                    New York, NY 10001
                  </p>
                </div>
              </div>
              <div className="flex items-start space-x-4">
                <div className="w-12 h-12 bg-purple-600 rounded-full flex items-center justify-center flex-shrink-0">
                  <i className="fas fa-phone text-white"></i>
                </div>
                <div>
                  <h3 className="text-xl font-semibold mb-2 text-white">
                    Call Us
                  </h3>
                  <p className="text-gray-400">
                    +1 (555) 123-4567
                    <br />
                    Mon - Fri, 9AM - 6PM EST
                  </p>
                </div>
              </div>
              <div className="flex items-start space-x-4">
                <div className="w-12 h-12 bg-purple-600 rounded-full flex items-center justify-center flex-shrink-0">
                  <i className="fas fa-envelope text-white"></i>
                </div>
                <div>
                  <h3 className="text-xl font-semibold mb-2 text-white">
                    Email Us
                  </h3>
                  <p className="text-gray-400">
                    hello@creativestudio.com
                    <br />
                    We'll respond within 24 hours
                  </p>
                </div>
              </div>
            </div>
            <Card className="bg-gray-800 border-gray-700">
              <CardContent className="p-8">
                <form className="space-y-6">
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div>
                      <label className="block text-sm font-medium text-gray-300 mb-2">
                        First Name
                      </label>
                      <Input
                        className="bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm"
                        placeholder="John"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium text-gray-300 mb-2">
                        Last Name
                      </label>
                      <Input
                        className="bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm"
                        placeholder="Doe"
                      />
                    </div>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-300 mb-2">
                      Email
                    </label>
                    <Input
                      type="email"
                      className="bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm"
                      placeholder="john@example.com"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-300 mb-2">
                      Project Type
                    </label>
                    <div className="relative">
                      <select className="w-full bg-gray-700 border border-gray-600 text-white text-sm rounded-md px-3 py-2 appearance-none cursor-pointer">
                        <option>Brand Design</option>
                        <option>Web Development</option>
                        <option>Mobile App</option>
                        <option>3D Modeling</option>
                        <option>Motion Graphics</option>
                        <option>Other</option>
                      </select>
                      <i className="fas fa-chevron-down absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 pointer-events-none"></i>
                    </div>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-300 mb-2">
                      Message
                    </label>
                    <textarea
                      className="w-full bg-gray-700 border border-gray-600 text-white placeholder-gray-400 text-sm rounded-md px-3 py-2 h-32 resize-none"
                      placeholder="Tell us about your project..."
                    ></textarea>
                  </div>
                  <Button className="!rounded-button whitespace-nowrap w-full bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white cursor-pointer">
                    <i className="fas fa-paper-plane mr-2"></i>
                    Send Message
                  </Button>
                </form>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>
      {/* Footer */}
      <footer className="bg-gray-800 py-16">
        <div className="max-w-7xl mx-auto px-6">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 mb-12">
            <div className="space-y-4">
              <div className="text-2xl font-bold text-white">
                <i className="fas fa-cube mr-2 text-purple-400"></i>
                CreativeStudio
              </div>
              <p className="text-gray-400 leading-relaxed">
                Transforming ideas into extraordinary digital experiences
                through innovative design and cutting-edge technology.
              </p>
              <div className="flex space-x-4">
                <a
                  href="#"
                  className="w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer"
                >
                  <i className="fab fa-facebook-f text-white"></i>
                </a>
                <a
                  href="#"
                  className="w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer"
                >
                  <i className="fab fa-twitter text-white"></i>
                </a>
                <a
                  href="#"
                  className="w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer"
                >
                  <i className="fab fa-instagram text-white"></i>
                </a>
                <a
                  href="#"
                  className="w-10 h-10 bg-gray-700 hover:bg-purple-600 rounded-full flex items-center justify-center transition-colors cursor-pointer"
                >
                  <i className="fab fa-linkedin-in text-white"></i>
                </a>
              </div>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-white mb-4">
                Services
              </h3>
              <ul className="space-y-2">
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Brand Design
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Web Development
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Mobile Apps
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    3D Modeling
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Motion Graphics
                  </a>
                </li>
              </ul>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-white mb-4">Company</h3>
              <ul className="space-y-2">
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    About Us
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Our Team
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Careers
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Contact
                  </a>
                </li>
                <li>
                  <a
                    href="#"
                    className="text-gray-400 hover:text-white transition-colors cursor-pointer"
                  >
                    Blog
                  </a>
                </li>
              </ul>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-white mb-4">
                Newsletter
              </h3>
              <p className="text-gray-400 mb-4">
                Stay updated with our latest projects and insights.
              </p>
              <div className="flex">
                <Input
                  type="email"
                  placeholder="Enter your email"
                  className="bg-gray-700 border-gray-600 text-white placeholder-gray-400 text-sm rounded-r-none"
                />
                <Button className="!rounded-button rounded-l-none bg-purple-600 hover:bg-purple-700 text-white cursor-pointer">
                  <i className="fas fa-arrow-right"></i>
                </Button>
              </div>
            </div>
          </div>
          <div className="border-t border-gray-700 pt-8">
            <div className="flex flex-col md:flex-row justify-between items-center">
              <p className="text-gray-500 text-sm">
                © 2025 CreativeStudio. All rights reserved.
              </p>
              <div className="flex space-x-6 mt-4 md:mt-0">
                <a
                  href="#"
                  className="text-gray-500 hover:text-white text-sm transition-colors cursor-pointer"
                >
                  Privacy Policy
                </a>
                <a
                  href="#"
                  className="text-gray-500 hover:text-white text-sm transition-colors cursor-pointer"
                >
                  Terms of Service
                </a>
                <a
                  href="#"
                  className="text-gray-500 hover:text-white text-sm transition-colors cursor-pointer"
                >
                  Cookie Policy
                </a>
              </div>
            </div>
          </div>
        </div>
      </footer>
    </div>
  );
};
export default App;
