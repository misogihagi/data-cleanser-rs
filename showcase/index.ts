// The exported code uses Tailwind CSS. Install Tailwind CSS in your dev environment to ensure all styles work.

import React, { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

const App: React.FC = () => {
  const [currentSlide, setCurrentSlide] = useState(0);
  const [showScrollTop, setShowScrollTop] = useState(false);
  const [activeSection, setActiveSection] = useState("overview");

  const heroImages = [
    "https://readdy.ai/api/search-image?query=stunning%20digital%20art%20NFT%20collection%20featuring%20vibrant%20abstract%20geometric%20patterns%20with%20holographic%20effects%20against%20dark%20cosmic%20background%20with%20purple%20and%20blue%20gradients&width=1440&height=600&seq=hero1&orientation=landscape",
    "https://readdy.ai/api/search-image?query=premium%20digital%20art%20NFT%20collection%20showcasing%20futuristic%20cyberpunk%20characters%20with%20neon%20lighting%20effects%20against%20dark%20starry%20background%20with%20purple%20accents&width=1440&height=600&seq=hero2&orientation=landscape",
    "https://readdy.ai/api/search-image?query=exclusive%20digital%20art%20NFT%20collection%20displaying%20ethereal%20fantasy%20creatures%20with%20magical%20aura%20effects%20against%20dark%20mystical%20background%20with%20purple%20and%20blue%20tones&width=1440&height=600&seq=hero3&orientation=landscape",
  ];

  const galleryImages = [
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20featuring%20abstract%20geometric%20patterns%20with%20holographic%20effects%20on%20dark%20background%20with%20purple%20highlights&width=300&height=300&seq=gallery1&orientation=squarish",
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20showing%20futuristic%20cyberpunk%20character%20with%20neon%20lighting%20on%20dark%20background%20with%20purple%20accents&width=300&height=300&seq=gallery2&orientation=squarish",
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20displaying%20ethereal%20fantasy%20creature%20with%20magical%20aura%20on%20dark%20mystical%20background%20with%20purple%20tones&width=300&height=300&seq=gallery3&orientation=squarish",
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20featuring%20cosmic%20landscape%20with%20nebula%20effects%20on%20dark%20space%20background%20with%20purple%20and%20blue%20gradients&width=300&height=300&seq=gallery4&orientation=squarish",
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20showing%20abstract%20digital%20sculpture%20with%20metallic%20surfaces%20on%20dark%20background%20with%20purple%20lighting&width=300&height=300&seq=gallery5&orientation=squarish",
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20displaying%20futuristic%20cityscape%20with%20neon%20lights%20on%20dark%20night%20background%20with%20purple%20glow%20effects&width=300&height=300&seq=gallery6&orientation=squarish",
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20featuring%20mystical%20portal%20with%20energy%20effects%20on%20dark%20dimensional%20background%20with%20purple%20and%20blue%20swirls&width=300&height=300&seq=gallery7&orientation=squarish",
    "https://readdy.ai/api/search-image?query=digital%20art%20NFT%20artwork%20showing%20robotic%20entity%20with%20glowing%20elements%20on%20dark%20technological%20background%20with%20purple%20circuit%20patterns&width=300&height=300&seq=gallery8&orientation=squarish",
  ];

  const relatedProjects = [
    {
      title: "Crypto Punks Revival",
      description: "Modern interpretation of classic pixel art NFTs",
      category: "Pixel Art",
      image:
        "https://readdy.ai/api/search-image?query=pixel%20art%20NFT%20collection%20featuring%20retro%20gaming%20characters%20with%20vibrant%20colors%20on%20dark%20background%20with%20purple%20accents&width=400&height=300&seq=related1&orientation=landscape",
    },
    {
      title: "Metaverse Avatars",
      description: "3D character collection for virtual worlds",
      category: "3D Art",
      image:
        "https://readdy.ai/api/search-image?query=3D%20avatar%20NFT%20collection%20showing%20futuristic%20humanoid%20characters%20with%20metallic%20textures%20on%20dark%20background%20with%20purple%20lighting&width=400&height=300&seq=related2&orientation=landscape",
    },
    {
      title: "Abstract Dimensions",
      description: "Generative art exploring mathematical beauty",
      category: "Generative",
      image:
        "https://readdy.ai/api/search-image?query=generative%20art%20NFT%20collection%20featuring%20mathematical%20patterns%20with%20flowing%20curves%20on%20dark%20background%20with%20purple%20and%20blue%20gradients&width=400&height=300&seq=related3&orientation=landscape",
    },
  ];

  const timelineEvents = [
    {
      date: "2024-01-15",
      title: "Project Initiation",
      description: "Initial concept development and art direction planning",
      status: "completed",
    },
    {
      date: "2024-02-20",
      title: "Art Creation",
      description: "Digital artwork creation and refinement process",
      status: "completed",
    },
    {
      date: "2024-03-10",
      title: "Smart Contract Development",
      description: "Blockchain integration and contract deployment",
      status: "completed",
    },
    {
      date: "2024-04-05",
      title: "Collection Launch",
      description: "Official launch and community engagement",
      status: "completed",
    },
    {
      date: "2024-05-01",
      title: "Secondary Market",
      description: "Trading platform integration and marketplace listing",
      status: "in-progress",
    },
  ];

  useEffect(() => {
    const handleScroll = () => {
      setShowScrollTop(window.scrollY > 400);
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentSlide((prev) => (prev + 1) % heroImages.length);
    }, 5000);

    return () => clearInterval(interval);
  }, [heroImages.length]);

  const scrollToTop = () => {
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  const scrollToSection = (sectionId: string) => {
    const element = document.getElementById(sectionId);
    if (element) {
      element.scrollIntoView({ behavior: "smooth" });
      setActiveSection(sectionId);
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Fixed Header */}
      <header className="fixed top-0 left-0 right-0 z-50 bg-gray-900/95 backdrop-blur-sm border-b border-gray-800">
        <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <a
              href="https://readdy.ai/home/f93329ae-41e6-46f7-bd2e-a773d56e0fc4/f9ab130b-12ae-40db-9ad4-3a514bd76816"
              data-readdy="true"
              className="cursor-pointer"
            >
              <Button
                variant="ghost"
                size="sm"
                className="!rounded-button whitespace-nowrap"
              >
                <i className="fas fa-arrow-left mr-2"></i>
                Back to Portfolio
              </Button>
            </a>
            <Separator orientation="vertical" className="h-6" />
            <h1 className="text-xl font-bold">Digital Art Collection</h1>
          </div>
          <Button
            variant="outline"
            size="sm"
            className="!rounded-button whitespace-nowrap cursor-pointer"
          >
            <i className="fas fa-share-alt mr-2"></i>
            Share Project
          </Button>
        </div>
      </header>

      {/* Hero Section */}
      <section className="relative h-screen overflow-hidden">
        <div className="absolute inset-0">
          {heroImages.map((image, index) => (
            <div
              key={index}
              className={`absolute inset-0 transition-opacity duration-1000 ${
                index === currentSlide ? "opacity-100" : "opacity-0"
              }`}
            >
              <img
                src={image}
                alt={`Hero slide ${index + 1}`}
                className="w-full h-full object-cover object-top"
              />
            </div>
          ))}
          <div className="absolute inset-0 bg-gradient-to-r from-gray-900/80 via-gray-900/40 to-transparent"></div>
        </div>

        <div className="relative z-10 flex items-center h-full max-w-7xl mx-auto px-6">
          <div className="max-w-2xl">
            <Badge className="mb-4 bg-purple-600 hover:bg-purple-700">
              NFT Collection
            </Badge>
            <h1 className="text-6xl font-bold mb-6 leading-tight">
              Digital Art
              <span className="block text-purple-400">Collection</span>
            </h1>
            <p className="text-xl text-gray-300 mb-8 leading-relaxed">
              An exclusive collection of 10,000 unique digital artworks
              exploring the intersection of technology and creativity. Each
              piece is meticulously crafted with stunning visual effects and
              stored permanently on the blockchain.
            </p>
            <div className="flex space-x-4">
              <Button
                size="lg"
                className="!rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700"
              >
                <i className="fas fa-eye mr-2"></i>
                View Collection
              </Button>
              <Button
                variant="outline"
                size="lg"
                className="!rounded-button whitespace-nowrap cursor-pointer"
              >
                <i className="fas fa-chart-line mr-2"></i>
                View Analytics
              </Button>
            </div>
          </div>
        </div>

        {/* Slide Indicators */}
        <div className="absolute bottom-8 left-1/2 transform -translate-x-1/2 flex space-x-2">
          {heroImages.map((_, index) => (
            <button
              key={index}
              onClick={() => setCurrentSlide(index)}
              className={`w-3 h-3 rounded-full transition-all cursor-pointer ${
                index === currentSlide ? "bg-purple-500" : "bg-gray-500"
              }`}
            />
          ))}
        </div>
      </section>

      {/* Navigation Tabs */}
      <nav className="sticky top-20 z-40 bg-gray-900/95 backdrop-blur-sm border-b border-gray-800">
        <div className="max-w-7xl mx-auto px-6">
          <div className="flex space-x-8">
            {[
              { id: "overview", label: "Overview", icon: "fas fa-info-circle" },
              { id: "gallery", label: "Gallery", icon: "fas fa-images" },
              { id: "technology", label: "Technology", icon: "fas fa-cog" },
              { id: "timeline", label: "Timeline", icon: "fas fa-clock" },
              {
                id: "testimonial",
                label: "Testimonial",
                icon: "fas fa-quote-left",
              },
              { id: "related", label: "Related", icon: "fas fa-link" },
            ].map((tab) => (
              <button
                key={tab.id}
                onClick={() => scrollToSection(tab.id)}
                className={`py-4 px-2 border-b-2 transition-colors cursor-pointer ${
                  activeSection === tab.id
                    ? "border-purple-500 text-purple-400"
                    : "border-transparent text-gray-400 hover:text-white"
                }`}
              >
                <i className={`${tab.icon} mr-2`}></i>
                {tab.label}
              </button>
            ))}
          </div>
        </div>
      </nav>

      {/* Project Overview */}
      <section id="overview" className="py-20 bg-gray-900">
        <div className="max-w-7xl mx-auto px-6">
          <div className="grid lg:grid-cols-2 gap-12 items-start">
            <div>
              <h2 className="text-4xl font-bold mb-6">Project Overview</h2>
              <div className="prose prose-invert max-w-none">
                <p className="text-lg text-gray-300 mb-6">
                  The Digital Art Collection represents a groundbreaking fusion
                  of traditional artistic principles with cutting-edge
                  blockchain technology. This project showcases 10,000 unique
                  digital artworks, each algorithmically generated while
                  maintaining artistic integrity and visual appeal.
                </p>
                <p className="text-lg text-gray-300 mb-6">
                  Our team collaborated with renowned digital artists to create
                  a collection that pushes the boundaries of what's possible in
                  the NFT space. Each artwork features intricate details,
                  vibrant colors, and unique characteristics that make every
                  piece truly one-of-a-kind.
                </p>
                <p className="text-lg text-gray-300">
                  The collection has gained significant traction in the NFT
                  community, with collectors appreciating both the artistic
                  quality and the technical innovation behind each piece. The
                  smart contract ensures true ownership and provenance for every
                  artwork.
                </p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-6">
              <Card className="bg-gray-800 border-gray-700">
                <CardHeader className="pb-3">
                  <CardTitle className="text-2xl text-purple-400">
                    10,000
                  </CardTitle>
                  <CardDescription>Total Collection Size</CardDescription>
                </CardHeader>
              </Card>
              <Card className="bg-gray-800 border-gray-700">
                <CardHeader className="pb-3">
                  <CardTitle className="text-2xl text-purple-400">
                    2,847 ETH
                  </CardTitle>
                  <CardDescription>Total Trading Volume</CardDescription>
                </CardHeader>
              </Card>
              <Card className="bg-gray-800 border-gray-700">
                <CardHeader className="pb-3">
                  <CardTitle className="text-2xl text-purple-400">
                    3,421
                  </CardTitle>
                  <CardDescription>Unique Owners</CardDescription>
                </CardHeader>
              </Card>
              <Card className="bg-gray-800 border-gray-700">
                <CardHeader className="pb-3">
                  <CardTitle className="text-2xl text-purple-400">
                    0.85 ETH
                  </CardTitle>
                  <CardDescription>Current Floor Price</CardDescription>
                </CardHeader>
              </Card>
            </div>
          </div>
        </div>
      </section>

      {/* Gallery Section */}
      <section id="gallery" className="py-20 bg-gray-800">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold mb-4">Interactive Gallery</h2>
            <p className="text-xl text-gray-300">
              Explore the stunning artworks in our collection
            </p>
          </div>

          <div className="grid grid-cols-4 gap-6">
            {galleryImages.map((image, index) => (
              <div
                key={index}
                className="group relative overflow-hidden rounded-lg cursor-pointer"
              >
                <img
                  src={image}
                  alt={`Gallery artwork ${index + 1}`}
                  className="w-full h-full object-cover object-top transition-transform duration-300 group-hover:scale-110"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-gray-900/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300">
                  <div className="absolute bottom-4 left-4 right-4">
                    <h3 className="text-white font-semibold">
                      Artwork #{index + 1}
                    </h3>
                    <p className="text-gray-300 text-sm">
                      Digital Art Collection
                    </p>
                  </div>
                </div>
              </div>
            ))}
          </div>

          <div className="text-center mt-12">
            <Button
              size="lg"
              className="!rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700"
            >
              <i className="fas fa-external-link-alt mr-2"></i>
              View Full Collection
            </Button>
          </div>
        </div>
      </section>

      {/* Technology Stack */}
      <section id="technology" className="py-20 bg-gray-900">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold mb-4">Technology Stack</h2>
            <p className="text-xl text-gray-300">
              Built with cutting-edge blockchain technology
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            <Card className="bg-gray-800 border-gray-700">
              <CardHeader>
                <div className="w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4">
                  <i className="fab fa-ethereum text-white text-xl"></i>
                </div>
                <CardTitle>Ethereum Blockchain</CardTitle>
                <CardDescription>
                  Secure and decentralized network for NFT storage
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="bg-gray-800 border-gray-700">
              <CardHeader>
                <div className="w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4">
                  <i className="fas fa-file-contract text-white text-xl"></i>
                </div>
                <CardTitle>ERC-721 Standard</CardTitle>
                <CardDescription>
                  Industry-standard smart contract implementation
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="bg-gray-800 border-gray-700">
              <CardHeader>
                <div className="w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4">
                  <i className="fas fa-database text-white text-xl"></i>
                </div>
                <CardTitle>IPFS Storage</CardTitle>
                <CardDescription>
                  Distributed file system for permanent storage
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="bg-gray-800 border-gray-700">
              <CardHeader>
                <div className="w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4">
                  <i className="fas fa-shield-alt text-white text-xl"></i>
                </div>
                <CardTitle>Security Audited</CardTitle>
                <CardDescription>
                  Comprehensive security audit by leading firms
                </CardDescription>
              </CardHeader>
            </Card>
          </div>

          <div className="mt-12 bg-gray-800 rounded-lg p-8">
            <h3 className="text-2xl font-bold mb-6">
              Technical Specifications
            </h3>
            <div className="grid md:grid-cols-2 gap-8">
              <div>
                <h4 className="text-lg font-semibold mb-4 text-purple-400">
                  Smart Contract Details
                </h4>
                <ul className="space-y-2 text-gray-300">
                  <li>
                    <strong>Contract Address:</strong> 0x1234...5678
                  </li>
                  <li>
                    <strong>Token Standard:</strong> ERC-721
                  </li>
                  <li>
                    <strong>Total Supply:</strong> 10,000 tokens
                  </li>
                  <li>
                    <strong>Mint Price:</strong> 0.08 ETH
                  </li>
                </ul>
              </div>
              <div>
                <h4 className="text-lg font-semibold mb-4 text-purple-400">
                  Metadata & Storage
                </h4>
                <ul className="space-y-2 text-gray-300">
                  <li>
                    <strong>Image Format:</strong> PNG (2048x2048)
                  </li>
                  <li>
                    <strong>Metadata:</strong> JSON on IPFS
                  </li>
                  <li>
                    <strong>Traits:</strong> 150+ unique attributes
                  </li>
                  <li>
                    <strong>Rarity:</strong> Algorithmically determined
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Project Timeline */}
      <section id="timeline" className="py-20 bg-gray-800">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold mb-4">Project Timeline</h2>
            <p className="text-xl text-gray-300">
              Key milestones in our development journey
            </p>
          </div>

          <div className="relative">
            <div className="absolute left-1/2 transform -translate-x-1/2 w-1 h-full bg-gray-700"></div>

            {timelineEvents.map((event, index) => (
              <div
                key={index}
                className={`relative flex items-center mb-12 ${index % 2 === 0 ? "justify-start" : "justify-end"}`}
              >
                <div className={`w-1/2 ${index % 2 === 0 ? "pr-8" : "pl-8"}`}>
                  <Card className="bg-gray-900 border-gray-700">
                    <CardHeader>
                      <div className="flex items-center justify-between mb-2">
                        <Badge
                          variant={
                            event.status === "completed"
                              ? "default"
                              : "secondary"
                          }
                        >
                          {event.status === "completed"
                            ? "Completed"
                            : "In Progress"}
                        </Badge>
                        <span className="text-sm text-gray-400">
                          {event.date}
                        </span>
                      </div>
                      <CardTitle className="text-lg">{event.title}</CardTitle>
                      <CardDescription>{event.description}</CardDescription>
                    </CardHeader>
                  </Card>
                </div>

                <div className="absolute left-1/2 transform -translate-x-1/2 w-4 h-4 bg-purple-600 rounded-full border-4 border-gray-800"></div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Client Testimonial */}
      <section id="testimonial" className="py-20 bg-gray-900">
        <div className="max-w-4xl mx-auto px-6 text-center">
          <h2 className="text-4xl font-bold mb-12">Client Testimonial</h2>

          <Card className="bg-gray-800 border-gray-700 p-8">
            <CardContent className="pt-6">
              <div className="flex justify-center mb-6">
                {[...Array(5)].map((_, i) => (
                  <i
                    key={i}
                    className="fas fa-star text-yellow-400 text-xl mr-1"
                  ></i>
                ))}
              </div>

              <blockquote className="text-2xl text-gray-300 mb-8 italic leading-relaxed">
                "Working with this team on our Digital Art Collection was an
                incredible experience. They delivered beyond our expectations,
                creating a truly unique and valuable NFT collection that
                resonated with our community. The technical implementation was
                flawless, and the artistic quality exceeded industry standards."
              </blockquote>

              <div className="flex items-center justify-center">
                <Avatar className="w-16 h-16 mr-4">
                  <AvatarImage src="https://readdy.ai/api/search-image?query=professional%20business%20executive%20portrait%20with%20confident%20expression%20on%20neutral%20background&width=64&height=64&seq=client1&orientation=squarish" />
                  <AvatarFallback>JD</AvatarFallback>
                </Avatar>
                <div className="text-left">
                  <h4 className="text-xl font-semibold">John Davidson</h4>
                  <p className="text-gray-400">CEO, CryptoArt Studios</p>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Related Projects */}
      <section id="related" className="py-20 bg-gray-800">
        <div className="max-w-7xl mx-auto px-6">
          <div className="text-center mb-12">
            <h2 className="text-4xl font-bold mb-4">Related Projects</h2>
            <p className="text-xl text-gray-300">
              Explore our other NFT collections
            </p>
          </div>

          <div className="grid md:grid-cols-3 gap-8">
            {relatedProjects.map((project, index) => (
              <Card
                key={index}
                className="bg-gray-900 border-gray-700 overflow-hidden group cursor-pointer hover:border-purple-500 transition-colors"
              >
                <div className="relative overflow-hidden">
                  <img
                    src={project.image}
                    alt={project.title}
                    className="w-full h-48 object-cover object-top group-hover:scale-105 transition-transform duration-300"
                  />
                  <Badge className="absolute top-4 left-4 bg-purple-600">
                    {project.category}
                  </Badge>
                </div>
                <CardHeader>
                  <CardTitle className="group-hover:text-purple-400 transition-colors">
                    {project.title}
                  </CardTitle>
                  <CardDescription>{project.description}</CardDescription>
                </CardHeader>
                <CardContent>
                  <Button
                    variant="outline"
                    className="w-full !rounded-button whitespace-nowrap cursor-pointer"
                  >
                    <i className="fas fa-arrow-right mr-2"></i>
                    View Project
                  </Button>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* Call-to-Action Section */}
      <section className="py-20 bg-gradient-to-r from-purple-900 to-blue-900">
        <div className="max-w-4xl mx-auto px-6 text-center">
          <h2 className="text-4xl font-bold mb-6">
            Ready to Start Your NFT Project?
          </h2>
          <p className="text-xl text-gray-300 mb-12">
            Let's create something extraordinary together. Our team specializes
            in developing unique NFT collections that stand out in the
            marketplace.
          </p>

          <div className="grid md:grid-cols-2 gap-12 items-center">
            <div className="text-left">
              <h3 className="text-2xl font-bold mb-4">What We Offer</h3>
              <ul className="space-y-3 text-gray-300">
                <li className="flex items-center">
                  <i className="fas fa-check text-purple-400 mr-3"></i>
                  Custom NFT Collection Design
                </li>
                <li className="flex items-center">
                  <i className="fas fa-check text-purple-400 mr-3"></i>
                  Smart Contract Development
                </li>
                <li className="flex items-center">
                  <i className="fas fa-check text-purple-400 mr-3"></i>
                  Marketplace Integration
                </li>
                <li className="flex items-center">
                  <i className="fas fa-check text-purple-400 mr-3"></i>
                  Community Building Support
                </li>
              </ul>
            </div>

            <Card className="bg-gray-800/50 border-gray-700">
              <CardHeader>
                <CardTitle>Get Started Today</CardTitle>
                <CardDescription>
                  Tell us about your project requirements
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <Input
                  placeholder="Your Name"
                  className="bg-gray-900 border-gray-700 text-white placeholder-gray-400"
                />
                <Input
                  placeholder="Email Address"
                  type="email"
                  className="bg-gray-900 border-gray-700 text-white placeholder-gray-400"
                />
                <div className="relative">
                  <select className="w-full p-3 bg-gray-900 border border-gray-700 rounded-md text-white appearance-none cursor-pointer">
                    <option>Select Project Type</option>
                    <option>NFT Collection</option>
                    <option>Gaming NFTs</option>
                    <option>Art Collection</option>
                    <option>Utility NFTs</option>
                  </select>
                  <i className="fas fa-chevron-down absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400"></i>
                </div>
                <Textarea
                  placeholder="Tell us about your project..."
                  className="bg-gray-900 border-gray-700 text-white placeholder-gray-400 min-h-[100px]"
                />
                <Button className="w-full !rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700">
                  <i className="fas fa-rocket mr-2"></i>
                  Start My Project
                </Button>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* Scroll to Top Button */}
      {showScrollTop && (
        <Button
          onClick={scrollToTop}
          className="fixed bottom-8 right-8 z-50 !rounded-button whitespace-nowrap cursor-pointer bg-purple-600 hover:bg-purple-700 w-12 h-12 p-0"
          size="sm"
        >
          <i className="fas fa-arrow-up"></i>
        </Button>
      )}
    </div>
  );
};

export default App;
