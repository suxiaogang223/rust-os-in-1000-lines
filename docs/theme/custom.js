// Rust OS 教程自定义JavaScript

document.addEventListener('DOMContentLoaded', function() {
    // 初始化功能
    initTableOfContents();
    initCodeCopyButtons();
    initChapterNavigation();
    initSearchEnhancement();
    initThemeToggle();
    initProgressBar();
    initSmoothScrolling();
});

// 目录功能增强
function initTableOfContents() {
    // 创建目录按钮
    const tocButton = document.createElement('button');
    tocButton.innerHTML = '<i class="fa fa-list"></i> 目录';
    tocButton.className = 'toc-button';
    tocButton.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        z-index: 1000;
        background: var(--md-primary-fg-color);
        color: white;
        border: none;
        border-radius: 6px;
        padding: 0.5rem 1rem;
        font-size: 0.9rem;
        cursor: pointer;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
        transition: all 0.3s ease;
    `;
    
    // 创建目录面板
    const tocPanel = document.createElement('div');
    tocPanel.className = 'toc-panel';
    tocPanel.style.cssText = `
        position: fixed;
        top: 0;
        right: -400px;
        width: 400px;
        height: 100vh;
        background: var(--md-default-bg-color);
        border-left: 1px solid var(--md-default-fg-color--lightest);
        box-shadow: -2px 0 8px rgba(0, 0, 0, 0.1);
        z-index: 1001;
        transition: right 0.3s ease;
        overflow-y: auto;
        padding: 2rem 1rem;
    `;
    
    // 生成目录内容
    const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
    const tocContent = document.createElement('div');
    tocContent.className = 'toc-content';
    
    headings.forEach((heading, index) => {
        const level = parseInt(heading.tagName.charAt(1));
        const id = heading.id || `heading-${index}`;
        heading.id = id;
        
        const tocItem = document.createElement('div');
        tocItem.className = `toc-item level-${level}`;
        tocItem.style.cssText = `
            margin-left: ${(level - 1) * 1.5}rem;
            margin-bottom: 0.5rem;
            padding: 0.3rem 0.5rem;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.3s ease;
        `;
        
        tocItem.innerHTML = `
            <a href="#${id}" style="text-decoration: none; color: var(--md-default-fg-color);">
                ${heading.textContent}
            </a>
        `;
        
        tocItem.addEventListener('click', function(e) {
            e.preventDefault();
            heading.scrollIntoView({ behavior: 'smooth' });
            closeTocPanel();
        });
        
        tocItem.addEventListener('mouseenter', function() {
            this.style.background = 'var(--md-primary-fg-color--transparent)';
        });
        
        tocItem.addEventListener('mouseleave', function() {
            this.style.background = 'transparent';
        });
        
        tocContent.appendChild(tocItem);
    });
    
    tocPanel.appendChild(tocContent);
    document.body.appendChild(tocPanel);
    document.body.appendChild(tocButton);
    
    // 切换目录面板
    tocButton.addEventListener('click', function() {
        if (tocPanel.style.right === '0px') {
            closeTocPanel();
        } else {
            openTocPanel();
        }
    });
    
    // 点击外部关闭目录
    document.addEventListener('click', function(e) {
        if (!tocPanel.contains(e.target) && !tocButton.contains(e.target)) {
            closeTocPanel();
        }
    });
    
    function openTocPanel() {
        tocPanel.style.right = '0px';
        tocButton.innerHTML = '<i class="fa fa-times"></i> 关闭';
    }
    
    function closeTocPanel() {
        tocPanel.style.right = '-400px';
        tocButton.innerHTML = '<i class="fa fa-list"></i> 目录';
    }
}

// 代码复制按钮
function initCodeCopyButtons() {
    const codeBlocks = document.querySelectorAll('pre code');
    
    codeBlocks.forEach(function(codeBlock) {
        const pre = codeBlock.parentElement;
        if (pre.tagName === 'PRE') {
            const copyButton = document.createElement('button');
            copyButton.innerHTML = '<i class="fa fa-copy"></i> 复制';
            copyButton.className = 'copy-button';
            copyButton.style.cssText = `
                position: absolute;
                top: 0.5rem;
                right: 0.5rem;
                background: var(--md-primary-fg-color);
                color: white;
                border: none;
                border-radius: 4px;
                padding: 0.3rem 0.6rem;
                font-size: 0.8rem;
                cursor: pointer;
                opacity: 0;
                transition: opacity 0.3s ease;
            `;
            
            pre.style.position = 'relative';
            pre.appendChild(copyButton);
            
            pre.addEventListener('mouseenter', function() {
                copyButton.style.opacity = '1';
            });
            
            pre.addEventListener('mouseleave', function() {
                copyButton.style.opacity = '0';
            });
            
            copyButton.addEventListener('click', function() {
                navigator.clipboard.writeText(codeBlock.textContent).then(function() {
                    copyButton.innerHTML = '<i class="fa fa-check"></i> 已复制';
                    setTimeout(function() {
                        copyButton.innerHTML = '<i class="fa fa-copy"></i> 复制';
                    }, 2000);
                });
            });
        }
    });
}

// 章节导航
function initChapterNavigation() {
    const content = document.querySelector('.md-content__inner');
    if (!content) return;
    
    const headings = content.querySelectorAll('h1, h2, h3, h4, h5, h6');
    if (headings.length < 2) return;
    
    // 创建章节导航
    const nav = document.createElement('div');
    nav.className = 'chapter-nav';
    nav.innerHTML = `
        <div class="nav-prev">
            <a href="#" id="prev-chapter">
                <i class="fa fa-chevron-left"></i> 上一章
            </a>
        </div>
        <div class="nav-current">
            <span id="current-chapter">当前章节</span>
        </div>
        <div class="nav-next">
            <a href="#" id="next-chapter">
                下一章 <i class="fa fa-chevron-right"></i>
            </a>
        </div>
    `;
    
    content.insertBefore(nav, content.firstChild);
    
    // 更新当前章节
    function updateCurrentChapter() {
        const scrollTop = window.pageYOffset;
        let currentHeading = headings[0];
        
        headings.forEach(function(heading) {
            if (heading.offsetTop <= scrollTop + 100) {
                currentHeading = heading;
            }
        });
        
        document.getElementById('current-chapter').textContent = currentHeading.textContent;
    }
    
    window.addEventListener('scroll', updateCurrentChapter);
    updateCurrentChapter();
}

// 搜索增强
function initSearchEnhancement() {
    const searchInput = document.querySelector('.md-search__input');
    if (searchInput) {
        searchInput.placeholder = '搜索教程内容...';
        
        // 添加搜索建议
        searchInput.addEventListener('input', function() {
            const query = this.value.toLowerCase();
            if (query.length > 2) {
                // 这里可以添加搜索建议逻辑
                console.log('搜索:', query);
            }
        });
    }
}

// 主题切换
function initThemeToggle() {
    const themeButton = document.createElement('button');
    themeButton.innerHTML = '<i class="fa fa-moon"></i>';
    themeButton.className = 'theme-toggle';
    themeButton.style.cssText = `
        position: fixed;
        top: 20px;
        right: 80px;
        z-index: 1000;
        background: var(--md-primary-fg-color);
        color: white;
        border: none;
        border-radius: 50%;
        width: 40px;
        height: 40px;
        cursor: pointer;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
        transition: all 0.3s ease;
    `;
    
    document.body.appendChild(themeButton);
    
    themeButton.addEventListener('click', function() {
        const html = document.documentElement;
        const currentTheme = html.getAttribute('data-md-color-scheme');
        const newTheme = currentTheme === 'slate' ? 'default' : 'slate';
        
        html.setAttribute('data-md-color-scheme', newTheme);
        localStorage.setItem('mdbook-theme', newTheme);
        
        this.innerHTML = newTheme === 'slate' ? '<i class="fa fa-sun"></i>' : '<i class="fa fa-moon"></i>';
    });
    
    // 恢复主题设置
    const savedTheme = localStorage.getItem('mdbook-theme');
    if (savedTheme) {
        document.documentElement.setAttribute('data-md-color-scheme', savedTheme);
        themeButton.innerHTML = savedTheme === 'slate' ? '<i class="fa fa-sun"></i>' : '<i class="fa fa-moon"></i>';
    }
}

// 进度条
function initProgressBar() {
    const progressBar = document.createElement('div');
    progressBar.className = 'progress-bar';
    progressBar.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 0%;
        height: 3px;
        background: var(--md-primary-fg-color);
        z-index: 1002;
        transition: width 0.3s ease;
    `;
    
    document.body.appendChild(progressBar);
    
    function updateProgress() {
        const scrollTop = window.pageYOffset;
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        const scrollPercent = (scrollTop / docHeight) * 100;
        progressBar.style.width = scrollPercent + '%';
    }
    
    window.addEventListener('scroll', updateProgress);
    updateProgress();
}

// 平滑滚动
function initSmoothScrolling() {
    const links = document.querySelectorAll('a[href^="#"]');
    
    links.forEach(function(link) {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);
            
            if (targetElement) {
                targetElement.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
}

// 键盘快捷键
document.addEventListener('keydown', function(e) {
    // Ctrl/Cmd + K 打开搜索
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        const searchInput = document.querySelector('.md-search__input');
        if (searchInput) {
            searchInput.focus();
        }
    }
    
    // ESC 关闭目录
    if (e.key === 'Escape') {
        const tocPanel = document.querySelector('.toc-panel');
        if (tocPanel && tocPanel.style.right === '0px') {
            const tocButton = document.querySelector('.toc-button');
            if (tocButton) {
                tocButton.click();
            }
        }
    }
});

// 响应式处理
function handleResize() {
    const tocPanel = document.querySelector('.toc-panel');
    if (tocPanel && window.innerWidth < 768) {
        tocPanel.style.width = '100vw';
        tocPanel.style.right = '-100vw';
    } else if (tocPanel) {
        tocPanel.style.width = '400px';
    }
}

window.addEventListener('resize', handleResize);
handleResize();

// 页面加载完成后的处理
window.addEventListener('load', function() {
    // 添加加载动画
    document.body.style.opacity = '0';
    document.body.style.transition = 'opacity 0.5s ease';
    
    setTimeout(function() {
        document.body.style.opacity = '1';
    }, 100);
    
    // 初始化其他功能
    initLazyLoading();
    initTooltips();
});

// 懒加载图片
function initLazyLoading() {
    const images = document.querySelectorAll('img[data-src]');
    const imageObserver = new IntersectionObserver(function(entries) {
        entries.forEach(function(entry) {
            if (entry.isIntersecting) {
                const img = entry.target;
                img.src = img.dataset.src;
                img.classList.remove('lazy');
                imageObserver.unobserve(img);
            }
        });
    });
    
    images.forEach(function(img) {
        imageObserver.observe(img);
    });
}

// 工具提示
function initTooltips() {
    const elements = document.querySelectorAll('[data-tooltip]');
    
    elements.forEach(function(element) {
        element.addEventListener('mouseenter', function() {
            const tooltip = document.createElement('div');
            tooltip.className = 'tooltip';
            tooltip.textContent = this.dataset.tooltip;
            tooltip.style.cssText = `
                position: absolute;
                background: var(--md-default-fg-color);
                color: var(--md-default-bg-color);
                padding: 0.5rem;
                border-radius: 4px;
                font-size: 0.8rem;
                z-index: 1000;
                pointer-events: none;
            `;
            
            document.body.appendChild(tooltip);
            
            const rect = this.getBoundingClientRect();
            tooltip.style.left = rect.left + 'px';
            tooltip.style.top = (rect.top - tooltip.offsetHeight - 5) + 'px';
            
            this.tooltip = tooltip;
        });
        
        element.addEventListener('mouseleave', function() {
            if (this.tooltip) {
                this.tooltip.remove();
                this.tooltip = null;
            }
        });
    });
}
