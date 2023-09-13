import React from 'react';
import {
  ChatMultipleRegular,
  LiveRegular,
  MicRegular,
  VideoRegular,
} from '@fluentui/react-icons';
import ThemedImage from '@theme/ThemedImage';
import clsx from 'clsx';


import Link from '@docusaurus/Link';

import { Linkedin, Twitter } from 'react-feather';
import { DiscordIcon, GithubIcon } from '@site/src/icons';


export default function HeroSection() {
  
  return (
    <>
      <section className="noise-bg no-underline-links lg:py-0 px-24 pt-96">
        <div className="flex flex-col justify-between py-14 text-center pt-16">
          <h1 className="text-6xl	mb-4 font-jakarta leading-relaxed text-5xl  text-secondary  font-extrabold tracking-tight ">
          <span className="text-primary">Workflows Automation</span>
            <br/>
            in the terminal 
          </h1>

          <p className="text-2xl text-text-400 font-light text-center mt-16 ">
            Share scripts and workflows with the community
          </p>

          {/* <div>
              <TechList items={items}/>
          </div> */}

        <div className="mt-16 mb-8 flex w-full flex-col items-center justify-center gap-2 text-sm font-semibold lg:flex-row lg:gap-8">
          <Link
            className="flex w-full items-center justify-center gap-2 rounded-sm  px-2 py-2 text-primary-100 lg:w-auto"
            href="https://community.cocmd.ai"
          >
            <DiscordIcon className="h-6 w-6" /> Discord
          </Link>
          <Link
            className="flex w-full items-center justify-center gap-2 rounded-sm  px-2 py-2 text-primary-100 lg:w-auto"
            href="https://community.cocmd.ai"
          >
            <GithubIcon className="h-6 w-6" /> Github
          </Link>
          <Link
            className="flex w-full items-center justify-center gap-2 rounded-sm border border-solid border-primary-100 px-2 py-2 text-primary-100 lg:w-auto"
            href="https://community.cocmd.ai"
          >
            Getting Started &rarr;
          </Link>
          </div>
          
        </div>
      </section>
      
    </>
  );
}
