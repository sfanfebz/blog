<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_4 minutes          Also available in   _7f05b7</name>
   <tag></tag>
   <elementGuidId>16cd49c2-0fcf-425a-8398-01cf7a0a2974</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.content</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Posts'])[1]/following::div[1]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>content</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                
  

    
      
        
          
          
        
        4 minutes

         | Also available in
          
              
          
        
      
    

    
      
        Migrate to Hugo from Jekyll
      

      

      

      
        Move static content to static
Jekyll has a rule that any directory not starting with _ will be copied as-is to the _site output. Hugo keeps all static content under static. You should therefore move it all there.
With Jekyll, something that looked like
▾ &lt;root>/
    ▾ images/
        logo.png

should become
▾ &lt;root>/
    ▾ static/
        ▾ images/
            logo.png

Additionally, you’ll want any files that should reside at the root (such as CNAME) to be moved to static.
Create your Hugo configuration file
Hugo can read your configuration as JSON, YAML or TOML. Hugo supports parameters custom configuration too. Refer to the Hugo configuration documentation for details.
Set your configuration publish folder to _site
The default is for Jekyll to publish to _site and for Hugo to publish to public. If, like me, you have _site mapped to a git submodule on the gh-pages branch, you’ll want to do one of two alternatives:


Change your submodule to point to map gh-pages to public instead of _site (recommended).
 git submodule deinit _site
 git rm _site
 git submodule add -b gh-pages git@github.com:your-username/your-repo.git public



Or, change the Hugo configuration to use _site instead of public.
 {
     ..
     &quot;publishdir&quot;: &quot;_site&quot;,
     ..
 }



Convert Jekyll templates to Hugo templates
That’s the bulk of the work right here. The documentation is your friend. You should refer to Jekyll’s template documentation if you need to refresh your memory on how you built your blog and Hugo’s template to learn Hugo’s way.
As a single reference data point, converting my templates for heyitsalex.net took me no more than a few hours.
Convert Jekyll plugins to Hugo shortcodes
Jekyll has plugins; Hugo has shortcodes. It’s fairly trivial to do a port.
Implementation
As an example, I was using a custom image_tag plugin to generate figures with caption when running Jekyll. As I read about shortcodes, I found Hugo had a nice built-in shortcode that does exactly the same thing.
Jekyll’s plugin:
module Jekyll
  class ImageTag &lt; Liquid::Tag
    @url = nil
    @caption = nil
    @class = nil
    @link = nil
    // Patterns
    IMAGE_URL_WITH_CLASS_AND_CAPTION =
    IMAGE_URL_WITH_CLASS_AND_CAPTION_AND_LINK = /(\w+)(\s+)((https?:\/\/|\/)(\S+))(\s+)&quot;(.*?)&quot;(\s+)->((https?:\/\/|\/)(\S+))(\s*)/i
    IMAGE_URL_WITH_CAPTION = /((https?:\/\/|\/)(\S+))(\s+)&quot;(.*?)&quot;/i
    IMAGE_URL_WITH_CLASS = /(\w+)(\s+)((https?:\/\/|\/)(\S+))/i
    IMAGE_URL = /((https?:\/\/|\/)(\S+))/i
    def initialize(tag_name, markup, tokens)
      super
      if markup =~ IMAGE_URL_WITH_CLASS_AND_CAPTION_AND_LINK
        @class   = $1
        @url     = $3
        @caption = $7
        @link = $9
      elsif markup =~ IMAGE_URL_WITH_CLASS_AND_CAPTION
        @class   = $1
        @url     = $3
        @caption = $7
      elsif markup =~ IMAGE_URL_WITH_CAPTION
        @url     = $1
        @caption = $5
      elsif markup =~ IMAGE_URL_WITH_CLASS
        @class = $1
        @url   = $3
      elsif markup =~ IMAGE_URL
        @url = $1
      end
    end
    def render(context)
      if @class
        source = &quot;&lt;figure class='#{@class}'>&quot;
      else
        source = &quot;&lt;figure>&quot;
      end
      if @link
        source += &quot;&lt;a href=\&quot;#{@link}\&quot;>&quot;
      end
      source += &quot;&lt;img src=\&quot;#{@url}\&quot;>&quot;
      if @link
        source += &quot;&lt;/a>&quot;
      end
      source += &quot;&lt;figcaption>#{@caption}&lt;/figcaption>&quot; if @caption
      source += &quot;&lt;/figure>&quot;
      source
    end
  end
end
Liquid::Template.register_tag('image', Jekyll::ImageTag)

is written as this Hugo shortcode:
&lt;!-- image -->
&lt;figure {{ with .Get &quot;class&quot; }}class=&quot;{{.}}&quot;{{ end }}>
    {{ with .Get &quot;link&quot;}}&lt;a href=&quot;{{.}}&quot;>{{ end }}
        &lt;img src=&quot;{{ .Get &quot;src&quot; }}&quot; {{ if or (.Get &quot;alt&quot;) (.Get &quot;caption&quot;) }}alt=&quot;{{ with .Get &quot;alt&quot;}}{{.}}{{else}}{{ .Get &quot;caption&quot; }}{{ end }}&quot;{{ end }} />
    {{ if .Get &quot;link&quot;}}&lt;/a>{{ end }}
    {{ if or (or (.Get &quot;title&quot;) (.Get &quot;caption&quot;)) (.Get &quot;attr&quot;)}}
    &lt;figcaption>{{ if isset .Params &quot;title&quot; }}
        {{ .Get &quot;title&quot; }}{{ end }}
        {{ if or (.Get &quot;caption&quot;) (.Get &quot;attr&quot;)}}&lt;p>
        {{ .Get &quot;caption&quot; }}
        {{ with .Get &quot;attrlink&quot;}}&lt;a href=&quot;{{.}}&quot;> {{ end }}
            {{ .Get &quot;attr&quot; }}
        {{ if .Get &quot;attrlink&quot;}}&lt;/a> {{ end }}
        &lt;/p> {{ end }}
    &lt;/figcaption>
    {{ end }}
&lt;/figure>
&lt;!-- image -->

Usage
I simply changed:
{% image full http://farm5.staticflickr.com/4136/4829260124_57712e570a_o_d.jpg &quot;One of my favorite touristy-type photos. I secretly waited for the good light while we were &quot;having fun&quot; and took this. Only regret: a stupid pole in the top-left corner of the frame I had to clumsily get rid of at post-processing.&quot; ->http://www.flickr.com/photos/alexnormand/4829260124/in/set-72157624547713078/ %}

to this (this example uses a slightly extended version named fig, different than the built-in figure):
{{% fig class=&quot;full&quot; src=&quot;http://farm5.staticflickr.com/4136/4829260124_57712e570a_o_d.jpg&quot; title=&quot;One of my favorite touristy-type photos. I secretly waited for the good light while we were having fun and took this. Only regret: a stupid pole in the top-left corner of the frame I had to clumsily get rid of at post-processing.&quot; link=&quot;http://www.flickr.com/photos/alexnormand/4829260124/in/set-72157624547713078/&quot; %}}

As a bonus, the shortcode named parameters are, arguably, more readable.
Finishing touches
Fix content
Depending on the amount of customization that was done with each post with Jekyll, this step will require more or less effort. There are no hard and fast rules here except that hugo server --watch is your friend. Test your changes and fix errors as needed.
Clean up
You’ll want to remove the Jekyll configuration at this point. If you have anything else that isn’t used, delete it.
A practical example in a diff
Hey, it’s Alex was migrated in less than a father-with-kids day from Jekyll to Hugo. You can see all the changes (and screw-ups) by looking at this diff.

      
    

    

    
      
      

      
        
          
          
          
          
          
        
        813 Words
      

      
        
          
          
          
          
        
        
          2014-03-10 00:00
        

         
          
            
              (Last updated: 2021-02-27 09:04)
            
          
        
      
        
          
            
            
            
          

          002d3da
          @ 2021-02-27
        
    
      
      
        

  
    
    
  




  
      
        
    
  




  
    
      
    
  




  
    
    
  




  
    
    
  




  
    
    
  




  
    
    
  




  
    
    
  




  
    
    
  




  
			
    
  




  
      
    
  


      

    
      
        
          
          
        

        
          
            
              
                ←
                Creating a New Theme
              
            
          

          
            
              
                (Hu)go Template Primer
                →
              
            
          
        
      
    


    

    

  

            </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;container&quot;]/div[@class=&quot;content&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Posts'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='About'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div</value>
   </webElementXpaths>
</WebElementEntity>
